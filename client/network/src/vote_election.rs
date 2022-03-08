#![allow(missing_docs)]
use crate::{
	config::{self, ProtocolId},
	error,
	service::NetworkService,
	utils::{interval, LruHashSet},
	Event, ExHashT, ObservedRole,
};

use codec::{Encode, Decode};
use bytes::Bytes;
use futures::{channel::mpsc, prelude::*};
use libp2p::{multiaddr, PeerId};
use prometheus_endpoint::{register, Counter, PrometheusError, Registry, U64};
use sp_runtime::traits::{Block as BlockT};
use std::{
	borrow::Cow,
	collections::{HashMap, hash_map::Entry},
	iter,
	num::NonZeroUsize,
	pin::Pin,
	sync::{
		Arc,
	},
	time,
};

use sp_consensus::{VoteData, VoteElectionRequest, ElectionData};
use sp_core::{blake2_256, H256};

const PROPAGATE_TIMEOUT: time::Duration = time::Duration::from_millis(3900);
const MAX_NOTIFICATION_SIZE: u64 = 16 * 1024 * 1024;
const MAX_KNOWN_NOTIFICATIONS: usize = 1024; // ~300kb per peer + overhead.
const MAX_PENDINGS: usize = 512;

struct Metrics {
	propagated_numbers: Counter<U64>,
}

impl Metrics {
	fn register(r: &Registry) -> Result<Self, PrometheusError> {
		Ok(Metrics {
			propagated_numbers: register(
				Counter::new(
					"sync_propaget_numbers",
					"Number of producer vote number propagated to at least one peer",
				)?,
				r,
			)?,
		})
	}
}

pub struct VoteElectionHandlerPrototype {
	protocol_name: Cow<'static, str>,
}

impl VoteElectionHandlerPrototype {
	/// Create a new instance.
	pub fn new(protocol_id: ProtocolId) -> Self {
		VoteElectionHandlerPrototype {
			protocol_name: Cow::from({
				let mut proto = String::new();
				proto.push_str("/");
				proto.push_str(protocol_id.as_ref());
				proto.push_str("/vote-electoin/1");
				proto
			}),
		}
	}

	/// Returns the configuration of the set to put in the network configuration.
	pub fn set_config(&self) -> config::NonDefaultSetConfig {
		config::NonDefaultSetConfig {
			notifications_protocol: self.protocol_name.clone(),
			fallback_names: Vec::new(),
			max_notification_size: MAX_NOTIFICATION_SIZE,
			set_config: config::SetConfig {
				in_peers: 0,
				out_peers: 0,
				reserved_nodes: Vec::new(),
				non_reserved_mode: config::NonReservedPeerMode::Deny,
			},
		}
	}

	/// Turns the prototype into the actual handler. Returns a controller that allows controlling
	/// the behaviour of the handler while it's running.
	///
	/// Important: the transactions handler is initially disabled and doesn't gossip transactions.
	/// You must call [`TransactionsHandlerController::set_gossip_enabled`] to enable it.
	pub fn build<B: BlockT + 'static, H: ExHashT>(
		self,
		service: Arc<NetworkService<B, H>>,
		metrics_registry: Option<&Registry>,
	) -> error::Result<(VoteElectionHandler<B, H>, VoteElectionHandlerController<B>)> {
		let event_stream = service.event_stream("vote-election-handler").boxed();
		let (to_handler, from_controller) = mpsc::unbounded();
		// let gossip_enabled = Arc::new(AtomicBool::new(false));
		// let (vote_notification_tx, vote_notification_rx) = mpsc::unbounded();
		let (local_event_tx, local_event_rx) = mpsc::unbounded();

		let handler = VoteElectionHandler {
			protocol_name: self.protocol_name,
			propagate_timeout: Box::pin(interval(PROPAGATE_TIMEOUT)),
			pending_elections: Vec::with_capacity(MAX_PENDINGS),
			pending_votes: Vec::with_capacity(MAX_PENDINGS),
			service,
			event_stream,
			peers: HashMap::new(),
			from_controller,
			metrics: if let Some(r) = metrics_registry {
				Some(Metrics::register(r)?)
			} else {
				None
			},
			vote_notification_tx: None,
			election_notification_tx: None,
			local_event_tx: local_event_tx,
			local_event_rx: local_event_rx,
		};

		let controller = VoteElectionHandlerController { 
			to_handler,
		};

		Ok((handler, controller))
	}
}

pub struct VoteElectionHandlerController<B: BlockT>{
    to_handler: mpsc::UnboundedSender<ToHandler<B>>,
}

impl<B: BlockT> VoteElectionHandlerController<B>{
	pub fn handle_request(&self, request: VoteElectionRequest<B>){
		match request{
			VoteElectionRequest::BuildVoteStream(tx) =>{
				let _ = self.to_handler.unbounded_send(ToHandler::BuildVoteStream(tx));
			},
			VoteElectionRequest::BuildElectionStream(tx)=>{
				let _ = self.to_handler.unbounded_send(ToHandler::BuildElectionStream(tx));
			},
			VoteElectionRequest::PropagateVote(vote_data) => {
				let _ = self.to_handler.unbounded_send(ToHandler::PropagateVote(vote_data));
			},
			VoteElectionRequest::PropagateElection(election_data)=>{
				let _ = self.to_handler.unbounded_send(ToHandler::PropagateElection(election_data));
			},
		}
	}
}

#[derive(Debug)]
enum ToHandler<B: BlockT> {
	BuildVoteStream(mpsc::UnboundedSender<VoteData<B>>),
	BuildElectionStream(mpsc::UnboundedSender<ElectionData<B>>),
	PropagateVote(VoteData<B>),
	PropagateElection(ElectionData<B>),
}

/// Handler for transactions. Call [`TransactionsHandler::run`] to start the processing.
pub struct VoteElectionHandler<B: BlockT + 'static, H: ExHashT> {
	protocol_name: Cow<'static, str>,
	/// Interval at which we call `propagate_transactions`.
	propagate_timeout: Pin<Box<dyn Stream<Item = ()> + Send>>,

	pending_elections: Vec<ElectionData<B>>,
	pending_votes: Vec<VoteData<B>>,

	/// Network service to use to send messages and manage peers.
	service: Arc<NetworkService<B, H>>,
	/// Stream of networking events.
	event_stream: Pin<Box<dyn Stream<Item = Event> + Send>>,
	// All connected peers
	peers: HashMap<PeerId, Peer>,
	// transaction_pool: Arc<dyn TransactionPool<H, B>>,
	// local_role: config::Role,
	from_controller: mpsc::UnboundedReceiver<ToHandler<B>>,
	/// Prometheus metrics.
	metrics: Option<Metrics>,

	vote_notification_tx: Option<mpsc::UnboundedSender<VoteData<B>>>,
	election_notification_tx: Option<mpsc::UnboundedSender<ElectionData<B>>>,

	local_event_tx: mpsc::UnboundedSender<Event>,
	local_event_rx: mpsc::UnboundedReceiver<Event>,
}

#[derive(Encode, Decode, Debug)]
enum VoteElectionNotification<B: BlockT>{
	Vote(VoteData<B>),
	Election(ElectionData<B>),
}

/// Peer information
#[derive(Debug)]
struct Peer {
	known_votes: LruHashSet<H256>,
	known_elections: LruHashSet<H256>,
	// known_elections: LruHashSet<H256>,
	role: ObservedRole,
}

impl<B: BlockT + 'static, H: ExHashT> VoteElectionHandler<B, H> {
	/// Turns the [`TransactionsHandler`] into a future that should run forever and not be
	/// interrupted.
	pub async fn run(mut self) {
		loop {
			futures::select! {
				_ = self.propagate_timeout.next().fuse() => {
					// log::info!("propagate timeout");
					self.propagate_vote_and_election();
				},
				network_event = self.event_stream.next().fuse() => {
					if let Some(network_event) = network_event {
						self.handle_network_event(network_event).await;
					} else {
						// Networking has seemingly closed. Closing as well.
						return;
					}
				},
				// message from self
				self_event = self.local_event_rx.select_next_some() =>{
					self.handle_network_event(self_event).await;
				},
				message = self.from_controller.select_next_some() => {
					match message {
						ToHandler::BuildVoteStream(tx)=>{
							self.vote_notification_tx = Some(tx);
						},
						ToHandler::BuildElectionStream(tx)=>{
							self.election_notification_tx = Some(tx);
						}
						ToHandler::PropagateVote(vote_data) => {
							self.propagate_vote(vote_data);
						},
						ToHandler::PropagateElection(election_data)=>{
							self.propagate_election(election_data);
						},
					}
				},
			}
		}
	}

	async fn handle_network_event(&mut self, event: Event) {
		match event {
			Event::Dht(_) => {},
			Event::SyncConnected { remote } => {
				let addr = iter::once(multiaddr::Protocol::P2p(remote.into()))
					.collect::<multiaddr::Multiaddr>();
				let result = self.service.add_peers_to_reserved_set(
					self.protocol_name.clone(),
					iter::once(addr).collect(),
				);
				if let Err(err) = result {
					log::error!(target: "sync", "Add reserved peer failed: {}", err);
				}
			},
			Event::SyncDisconnected { remote } => {
				let addr = iter::once(multiaddr::Protocol::P2p(remote.into()))
					.collect::<multiaddr::Multiaddr>();
				let result = self.service.remove_peers_from_reserved_set(
					self.protocol_name.clone(),
					iter::once(addr).collect(),
				);
				if let Err(err) = result {
					log::error!(target: "sync", "Removing reserved peer failed: {}", err);
				}
				// self.service.remove_peers_from_reserved_set(
				// 	self.protocol_name.clone(), 
				// 	iter::once(remote).collect(),
				// );
			},

			Event::NotificationStreamOpened { remote, protocol, role, .. }
				if protocol == self.protocol_name =>
			{
				log::info!("insert new peer: {:?}", remote);
				let _was_in = self.peers.insert(
					remote,
					Peer {
						// known_transactions: LruHashSet::new(
						// 	NonZeroUsize::new(MAX_KNOWN_NOTIFICATIONS).expect("Constant is nonzero"),
						// ),
						known_elections: LruHashSet::new(
							NonZeroUsize::new(MAX_KNOWN_NOTIFICATIONS).expect("Constant is nonzero"),
						),
						known_votes: LruHashSet::new(
							NonZeroUsize::new(MAX_KNOWN_NOTIFICATIONS).expect("Constant is nonzero"),
						),
						role,
					},
				);
				debug_assert!(_was_in.is_none());
			}
			Event::NotificationStreamClosed { remote, protocol }
				if protocol == self.protocol_name =>
			{
				log::info!("remove peer: {:?}", remote);

				let _peer = self.peers.remove(&remote);
				debug_assert!(_peer.is_some());
			}

			Event::NotificationsReceived { remote: who, messages } => {
				for (protocol, message) in messages {
					if protocol != self.protocol_name {
						continue
					}

					if let Ok(msg) = <VoteElectionNotification<B> as Decode>::decode(&mut message.as_ref()){
						match msg {
							VoteElectionNotification::Vote(vote_data)=>{
								// log::info!("<<<< VoteElectionNotification:VoteV2");
								// log::info!("<<<< vote_v2: {:?} from: {:?}", vote_data, remote);

								// self.pending_votes.insert(vote_hash.clone(), vote_data.clone());
								let vote_hash = hash_of(&vote_data);

								if !self.pending_votes.contains(&vote_data){
									// send to consensus
									self.vote_notification_tx.as_ref().map(|v|{
										let _ = v.unbounded_send(vote_data.clone());
									});

									self.pending_votes.push(vote_data.clone());
									while self.pending_votes.len() > MAX_PENDINGS{
										self.pending_votes.remove(0);
									}
								}

								if let Some(ref mut peer) = self.peers.get_mut(&who) {
									peer.known_votes.insert(vote_hash.clone());
								}

							},
							VoteElectionNotification::Election(election_data)=>{
								let election_hash = hash_of(&election_data);

								if !self.pending_elections.contains(&election_data){
									// send to consensus
									self.election_notification_tx.as_ref().map(|v|{
										// log::info!("Election");
										let _ = v.unbounded_send(election_data.clone());
									});

									self.pending_elections.push(election_data.clone());
									while self.pending_elections.len() > MAX_PENDINGS{
										self.pending_elections.remove(0);
									}
								}

								if let Some(ref mut peer) = self.peers.get_mut(&who) {
									peer.known_elections.insert(election_hash.clone());
								}
							},
						}
					}
				}
			},

			// Not our concern.
			Event::NotificationStreamOpened { .. } | Event::NotificationStreamClosed { .. } => {},
		}
	}

	fn propagate_vote_and_election(&mut self){
		let pending_elections = self.pending_elections.clone();
		for election_data in pending_elections.iter(){
			self.propagate_election(election_data.clone());
		}

		let pending_votes = self.pending_votes.clone();
		for vote_data in pending_votes.iter(){
			self.propagate_vote(vote_data.clone());
		}
	}

	fn propagate_election(&mut self, election_data: ElectionData<B>){
		let mut propagated_numbers = 0;
		// let hash = election_data.hash.clone();
		let election_hash = hash_of(&election_data);
		let election_block_hash = election_data.block_hash.clone();
		let to_send = VoteElectionNotification::Election(election_data).encode();

		let (mut known_count ,mut unknown_count) = (0, 0);
		for (who, peer) in self.peers.iter_mut() {
			if matches!(peer.role, ObservedRole::Light) {
				continue;
			}

			if peer.known_elections.insert(election_hash){
				unknown_count += 1;
				propagated_numbers += 1;

				// log::info!(">>>> Election {:?}, client/network/src/producer_select.rs: 540", who);
				self.service.write_notification(
					who.clone(),
					self.protocol_name.clone(),
					to_send.clone(),
				);
			}
			else{
				known_count += 1;
				// log::info!("ignore node: {:?}, already known this election", who);
			}
		}
		// if unknown_count > 0{
		// 	log::info!(
		// 		"propagate election result: {} {}/{}",
		// 		election_block_hash,
		// 		unknown_count,
		// 		known_count + unknown_count
		// 	);
		// }

		// log::info!(">>>> Election to local_peer_id: client/network/src/producer_select.rs: 548");
		let local_peer_id = self.service.local_peer_id();
		let _ = self.local_event_tx.unbounded_send(
			Event::NotificationsReceived{
				remote: local_peer_id.clone(), 
				messages: vec![(self.protocol_name.clone(), Bytes::from(to_send.clone()))],
			}
		);

		// log::info!("♓ Propagate election({}) to {} peers", hash, propagated_numbers);
		if let Some(ref metriecs) = self.metrics {
			metriecs.propagated_numbers.inc_by(propagated_numbers as _)
		}
	}

	fn propagate_vote(&mut self, vote_data: VoteData<B>){
		// log::info!("{:?}", vote_data);
		let mut propagated_numbers = 0;
		// let hash = vote_data.hash.clone();

		let vote_hash = hash_of(&vote_data);
		let vote_block_hash = vote_data.block_hash.clone();
		let to_send = VoteElectionNotification::Vote(vote_data).encode();

		let (mut known_count ,mut unknown_count) = (0, 0);
		for (who, peer) in self.peers.iter_mut() {
			if matches!(peer.role, ObservedRole::Light) {
				continue;
			}

			if peer.known_votes.insert(vote_hash){
				propagated_numbers += 1;

				// log::info!(">>>> {:?}, client/network/src/producer_select.rs:514", who);
				self.service.write_notification(
					who.clone(),
					self.protocol_name.clone(),
					to_send.clone(),
				);
				unknown_count += 1;
			}
			else{
				known_count += 1;
			}
		}
		// log::info!("propagate vote result: {} {}/{}", vote_hash, unknown_count, known_count + unknown_count);
		// if unknown_count > 0{
		// 	log::info!(
		// 		"propagate vote result: {} {}/{}",
		// 		vote_block_hash,
		// 		unknown_count,
		// 		known_count + unknown_count
		// 	);
		// }

		// log::info!(">>>> to local_peer_id: client/network/src/producer_select.rs:522");
		let local_peer_id = self.service.local_peer_id();
		let _ = self.local_event_tx.unbounded_send(
			Event::NotificationsReceived{
				remote: local_peer_id.clone(), 
				messages: vec![(self.protocol_name.clone(), Bytes::from(to_send.clone()))],
			}
		);

		// ::info!("♓ Propagate vote ({}) to {} peers", hash, propagated_numbers);

		if let Some(ref metriecs) = self.metrics {
			metriecs.propagated_numbers.inc_by(propagated_numbers as _)
		}
	}

	// fn propagate_vote_v1(&mut self, vote_data: VoteData<B>){
	// 	// log::info!("{:?}", vote_data);
	// 	let mut propagated_numbers = 0;
	// 	// let hash = vote_data.hash.clone();

	// 	let to_send = VoteElectionNotification::Vote(vote_data).encode();

	// 	for (who, peer) in self.peers.iter_mut() {
	// 		if matches!(peer.role, ObservedRole::Light) {
	// 			continue;
	// 		}

	// 		propagated_numbers += 1;

	// 		// log::info!(">>>> {:?}, client/network/src/producer_select.rs:514", who);
	// 		self.service.write_notification(
	// 			who.clone(),
	// 			self.protocol_name.clone(),
	// 			to_send.clone(),
	// 		);
	// 	}

	// 	// log::info!(">>>> to local_peer_id: client/network/src/producer_select.rs:522");
	// 	let local_peer_id = self.service.local_peer_id();
	// 	let _ = self.local_event_tx.unbounded_send(
	// 		Event::NotificationsReceived{
	// 			remote: local_peer_id.clone(), 
	// 			messages: vec![(self.protocol_name.clone(), Bytes::from(to_send.clone()))],
	// 		}
	// 	);

	// 	// ::info!("♓ Propagate vote ({}) to {} peers", hash, propagated_numbers);
	// 	if let Some(ref metriecs) = self.metrics {
	// 		metriecs.propagated_numbers.inc_by(propagated_numbers as _)
	// 	}
	// }
}

fn hash_of<E>(data: &E)->H256
where
	E: Encode,
{
	let encoded = data.encode();
	blake2_256(&encoded).into()
}
