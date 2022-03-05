bootnode_peer_id="12D3KooWJThfTb9iRQooS1UwCLA3vpyiGYGwHyxswzK19v2ENgbm"
spec_file = "./tmp/RawAuraSpec.json"

# Node1
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N01 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N01 \
    --chain $spec_file \
    --port 30353 \
    --ws-port 9965 \
    --rpc-port 9953 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name N01 &>./tmp/N01.log &\
sleep 0.1s

# Node2
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N02 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N02 \
    --chain $spec_file \
    --port 30354 \
    --ws-port 9966 \
    --rpc-port 9954 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name N02 &>./tmp/N02.log &\
sleep 0.1s

# Node3
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N03 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N03 \
    --chain $spec_file \
    --port 30355 \
    --ws-port 9967 \
    --rpc-port 9955 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name N03 &>./tmp/N03.log &\
sleep 0.1s

# Node4
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N04 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N04 \
    --chain $spec_file \
    --port 30356 \
    --ws-port 9968 \
    --rpc-port 9956 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name N04 &>./tmp/N04.log &\
sleep 0.1s

# Node5
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N05 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N05 \
    --chain $spec_file \
    --port 30357 \
    --ws-port 9969 \
    --rpc-port 9957 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name N05 &>./tmp/N05.log &\
sleep 0.1s

# Node6
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N06 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N06 \
    --chain $spec_file \
    --port 30358 \
    --ws-port 9970 \
    --rpc-port 9958 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name N06 &>./tmp/N06.log &\
sleep 0.1s

# Node7
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N07 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N07 \
    --chain $spec_file \
    --port 30359 \
    --ws-port 9971 \
    --rpc-port 9959 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name N07 &>./tmp/N07.log &\
sleep 0.1s

# Node8
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N08 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N08 \
    --chain $spec_file \
    --port 30360 \
    --ws-port 9972 \
    --rpc-port 9960 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name N08 &>./tmp/N08.log &\
sleep 0.1s

# Node9
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N09 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N09 \
    --chain $spec_file \
    --port 30361 \
    --ws-port 9973 \
    --rpc-port 9961 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name N09 &>./tmp/N09.log &\
sleep 0.1s

# Node10
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N10 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N10 \
    --chain $spec_file \
    --port 30362 \
    --ws-port 9974 \
    --rpc-port 9962 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name N10 &>./tmp/N10.log &\
sleep 0.1s

# Node11
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N11 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N11 \
    --chain $spec_file \
    --port 30363 \
    --ws-port 9975 \
    --rpc-port 9963 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name N11 &>./tmp/N11.log &\
sleep 0.1s

# Node12
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N12 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N12 \
    --chain $spec_file \
    --port 30364 \
    --ws-port 9976 \
    --rpc-port 9964 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name N12 &>./tmp/N12.log &\
sleep 0.1s

# # Node13
# ./target/debug/frontier-template-node purge-chain --base-path ./tmp/N13 --chain local -y;\
# ./target/debug/frontier-template-node \
#     --base-path ./tmp/N13 \
#     --chain $spec_file \
#     --port 30365 \
#     --ws-port 9977 \
#     --rpc-port 9965 \
#     --rpc-methods Unsafe \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     --name N13 &>./tmp/N13.log &\
# sleep 0.1s

# # Node14
# ./target/debug/frontier-template-node purge-chain --base-path ./tmp/N14 --chain local -y;\
# ./target/debug/frontier-template-node \
#     --base-path ./tmp/N14 \
#     --chain $spec_file \
#     --port 30366 \
#     --ws-port 9978 \
#     --rpc-port 9966 \
#     --rpc-methods Unsafe \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     --name N14 &>./tmp/N14.log &\
# sleep 0.1s

# # Node15
# ./target/debug/frontier-template-node purge-chain --base-path ./tmp/N15 --chain local -y;\
# ./target/debug/frontier-template-node \
#     --base-path ./tmp/N15 \
#     --chain $spec_file \
#     --port 30367 \
#     --ws-port 9979 \
#     --rpc-port 9967 \
#     --rpc-methods Unsafe \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     --name N15 &>./tmp/N15.log &\
# sleep 0.1s

# # Node16
# ./target/debug/frontier-template-node purge-chain --base-path ./tmp/N16 --chain local -y;\
# ./target/debug/frontier-template-node \
#     --base-path ./tmp/N16 \
#     --chain $spec_file \
#     --port 30368 \
#     --ws-port 9980 \
#     --rpc-port 9968 \
#     --rpc-methods Unsafe \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     --name N16 &>./tmp/N16.log &\
# sleep 0.1s

# # Node17
# ./target/debug/frontier-template-node purge-chain --base-path ./tmp/N17 --chain local -y;\
# ./target/debug/frontier-template-node \
#     --base-path ./tmp/N17 \
#     --chain $spec_file \
#     --port 30369 \
#     --ws-port 9981 \
#     --rpc-port 9969 \
#     --rpc-methods Unsafe \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     --name N17 &>./tmp/N17.log &\
# sleep 0.1s

# # Node18
# ./target/debug/frontier-template-node purge-chain --base-path ./tmp/N18 --chain local -y;\
# ./target/debug/frontier-template-node \
#     --base-path ./tmp/N18 \
#     --chain $spec_file \
#     --port 30370 \
#     --ws-port 9982 \
#     --rpc-port 9970 \
#     --rpc-methods Unsafe \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     --name N18 &>./tmp/N18.log &\
# sleep 0.1s

# # Node19
# ./target/debug/frontier-template-node purge-chain --base-path ./tmp/N19 --chain local -y;\
# ./target/debug/frontier-template-node \
#     --base-path ./tmp/N19 \
#     --chain $spec_file \
#     --port 30371 \
#     --ws-port 9983 \
#     --rpc-port 9971 \
#     --rpc-methods Unsafe \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     --name N19 &>./tmp/N19.log &\
# sleep 0.1s

# # Node20
# ./target/debug/frontier-template-node purge-chain --base-path ./tmp/N20 --chain local -y;\
# ./target/debug/frontier-template-node \
#     --base-path ./tmp/N20 \
#     --chain $spec_file \
#     --port 30372 \
#     --ws-port 9984 \
#     --rpc-port 9972 \
#     --rpc-methods Unsafe \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     --name N20 &>./tmp/N20.log &\
# sleep 1s
