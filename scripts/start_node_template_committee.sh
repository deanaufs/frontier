bootnode_peer_id="12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"

# 启动节点1
# 按照生成的Raw配置文件启动frontier-template-node
./target/debug/frontier-template-node purge-chain --base-path ./tmp/alice --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/alice \
    --chain local \
    --alice \
    --port 30333 \
    --ws-port 9945 \
    --rpc-port 9933 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --validator \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    &>./tmp/alice.log &\
sleep 0.1s

./target/debug/frontier-template-node purge-chain --base-path ./tmp/bob --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/bob \
    --chain local \
    --bob \
    --port 30334 \
    --ws-port 9946 \
    --rpc-port 9934 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --validator \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    &>./tmp/bob.log &\
sleep 0.1s

# ./target/debug/frontier-template-node purge-chain --base-path ./tmp/charlie --chain local -y;\
# ./target/debug/frontier-template-node \
#     --base-path ./tmp/charlie \
#     --chain local \
#     --charlie \
#     --port 30335 \
#     --ws-port 9947 \
#     --rpc-port 9935 \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --validator \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     &>./tmp/charlie.log &\
# sleep 0.1s

# ./target/debug/frontier-template-node purge-chain --base-path ./tmp/dave --chain local -y;\
# ./target/debug/frontier-template-node \
#     --base-path ./tmp/dave \
#     --chain local \
#     --dave \
#     --port 30336 \
#     --ws-port 9948 \
#     --rpc-port 9936 \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --validator \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     &>./tmp/dave.log &\
# sleep 0.1s

# ./target/debug/frontier-template-node purge-chain --base-path ./tmp/eve --chain local -y;\
# ./target/debug/frontier-template-node \
#     --base-path ./tmp/eve \
#     --chain local \
#     --eve \
#     --port 30337 \
#     --ws-port 9949 \
#     --rpc-port 9937 \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --validator \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     &>./tmp/eve.log &\
# sleep 0.1s

# ./target/debug/frontier-template-node purge-chain --base-path ./tmp/ferdie --chain local -y;\
# ./target/debug/frontier-template-node \
#     --base-path ./tmp/ferdie \
#     --chain local \
#     --ferdie \
#     --port 30338 \
#     --ws-port 9950 \
#     --rpc-port 9938 \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --validator \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     &>./tmp/ferdie.log &\
# sleep 0.1s