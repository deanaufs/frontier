bootnode_peer_id="12D3KooWJThfTb9iRQooS1UwCLA3vpyiGYGwHyxswzK19v2ENgbm"
spec_file="./tmp/RawSpec.json"

# 启动节点1
# 按照生成的Raw配置文件启动node-template
./target/debug/node-template purge-chain --base-path ./tmp/C01 --chain local -y;\
./target/debug/node-template \
    --base-path ./tmp/C01 \
    --chain $spec_file \
    --port 30333 \
    --ws-port 9945 \
    --rpc-port 9933 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C01 &>./tmp/C01.log &\
sleep 0.1s

# 启动节点2
# 按照生成的Raw配置文件启动node-template
./target/debug/node-template purge-chain --base-path ./tmp/C02 --chain local -y;\
./target/debug/node-template \
    --base-path ./tmp/C02 \
    --chain $spec_file \
    --port 30334 \
    --ws-port 9946 \
    --rpc-port 9934 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name C02 &>./tmp/C02.log &\
sleep 0.1s

# 启动节点3
# 按照生成的Raw配置文件启动node-template
./target/debug/node-template purge-chain --base-path ./tmp/C03 --chain local -y;\
./target/debug/node-template \
    --base-path ./tmp/C03 \
    --chain $spec_file \
    --port 30335 \
    --ws-port 9947 \
    --rpc-port 9935 \
    --validator \
    --rpc-methods Unsafe \
--telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name C03 &>./tmp/C03.log &\
sleep 0.1s

# 启动节点4
# 按照生成的Raw配置文件启动node-template
./target/debug/node-template purge-chain --base-path ./tmp/C04 --chain local -y;\
./target/debug/node-template \
    --base-path ./tmp/C04 \
    --chain $spec_file \
    --port 30336 \
    --ws-port 9948 \
    --rpc-port 9936 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name C04 &>./tmp/C04.log &\
sleep 0.1s

# 启动节点5
# 按照生成的Raw配置文件启动node-template
./target/debug/node-template purge-chain --base-path ./tmp/C05 --chain local -y;\
./target/debug/node-template \
    --base-path ./tmp/C05 \
    --chain $spec_file \
    --port 30337 \
    --ws-port 9949 \
    --rpc-port 9937 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name C05 &>./tmp/C05.log &\
sleep 0.1s

# 启动节点6
# 按照生成的Raw配置文件启动node-template
./target/debug/node-template purge-chain --base-path ./tmp/C06 --chain local -y;\
./target/debug/node-template \
    --base-path ./tmp/C06 \
    --chain $spec_file \
    --port 30338 \
    --ws-port 9950 \
    --rpc-port 9938 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name C06 &>./tmp/C06.log &\
sleep 0.1s

# 启动节点7
# 按照生成的Raw配置文件启动node-template
./target/debug/node-template purge-chain --base-path ./tmp/C07 --chain local -y;\
./target/debug/node-template \
    --base-path ./tmp/C07 \
    --chain $spec_file \
    --port 30339 \
    --ws-port 9951 \
    --rpc-port 9939 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name C07 &>./tmp/C07.log &\
sleep 0.1s

# 启动节点8
# 按照生成的Raw配置文件启动node-template
./target/debug/node-template purge-chain --base-path ./tmp/C08 --chain local -y;\
./target/debug/node-template \
    --base-path ./tmp/C08 \
    --chain $spec_file \
    --port 30340 \
    --ws-port 9952 \
    --rpc-port 9940 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
    --name C08 &>./tmp/C08.log &\
sleep 0.1s

# # 启动节点9
# # 按照生成的Raw配置文件启动node-template
# ./target/debug/node-template purge-chain --base-path ./tmp/C09 --chain local -y;\
# ./target/debug/node-template \
#     --base-path ./tmp/C09 \
#     --chain $spec_file \
#     --port 30341 \
#     --ws-port 9953 \
#     --rpc-port 9941 \
#     --validator \
#     --rpc-methods Unsafe \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     --name C09 &>./tmp/C09.log &\
# sleep 0.1s

# # 启动节点10
# # 按照生成的Raw配置文件启动node-template
# ./target/debug/node-template purge-chain --base-path ./tmp/C10 --chain local -y;\
# ./target/debug/node-template \
#     --base-path ./tmp/C10 \
#     --chain $spec_file \
#     --port 30342 \
#     --ws-port 9954 \
#     --rpc-port 9942 \
#     --validator \
#     --rpc-methods Unsafe \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$bootnode_peer_id \
#     --name C10 &>./tmp/C10.log &\
# sleep 0.1s