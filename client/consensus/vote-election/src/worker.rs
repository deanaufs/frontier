use sp_timestamp::Timestamp;
use std::ops::Deref;

use sp_runtime::{
	traits::{HashFor, NumberFor},
};

/// The changes that need to applied to the storage to create the state for a block.
///
/// See [`sp_state_machine::StorageChanges`] for more information.
pub type StorageChanges<Transaction, Block> =
	sp_state_machine::StorageChanges<Transaction, HashFor<Block>, NumberFor<Block>>;

pub trait InherentDataProviderExt {
	/// The current timestamp that will be found in the [`InherentData`](`sp_inherents::InherentData`).
	fn timestamp(&self) -> Timestamp;
}

/// Small macro for implementing `InherentDataProviderExt` for inherent data provider tuple.
macro_rules! impl_inherent_data_provider_ext_tuple {
	( T $(, $TN:ident)* $( , )?) => {
		impl<T, $( $TN ),*>  InherentDataProviderExt for (T, $($TN),*)
		where
			T: Deref<Target = Timestamp>,
		{
			fn timestamp(&self) -> Timestamp {
				*self.0.deref()
			}
		}
	}
}

impl_inherent_data_provider_ext_tuple!(T,);
impl_inherent_data_provider_ext_tuple!(T, S);

/// A header which has been checked
pub enum CheckedHeader<H, S> {
	/// A header which is fully checked, including signature. This is the pre-header
	/// accompanied by the seal components.
	///
	/// Includes the digest item that encoded the seal.
	Checked(H, S),
}