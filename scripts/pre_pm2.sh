# 启动节点alice
./target/release/frontier-template-node purge-chain --base-path ./tmp/alice -y;\
> ./tmp/alice.log;\
pm2 start -n alice --log ./tmp/alice.log \
./target/release/frontier-template-node --\
    --alice \
    --base-path ./tmp/alice \
    --chain local \
    --port 1601 \
    --rpc-port 1851 \
    --ws-port 1801 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --validator \
    --rpc-cors all \
    --unsafe-rpc-external \
    --unsafe-ws-external \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001;
sleep 0.1s

# 启动节点bob
./target/release/frontier-template-node purge-chain --base-path ./tmp/bob -y;\
> ./tmp/bob.log;\
pm2 start -n bob --log ./tmp/bob.log \
./target/release/frontier-template-node --\
    --bob \
    --base-path ./tmp/bob \
    --chain local \
    --port 1602 \
    --rpc-port 1852 \
    --ws-port 1802 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --validator; 
sleep 0.1s 

# 启动节点charlie
./target/release/frontier-template-node purge-chain --base-path ./tmp/charlie -y;\
> ./tmp/charlie.log;\
pm2 start -n charlie --log ./tmp/charlie.log \
./target/release/frontier-template-node --\
    --charlie \
    --base-path ./tmp/charlie \
    --chain local \
    --port 1603 \
    --rpc-port 1853 \
    --ws-port 1803 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --validator; 
sleep 0.1s 

# # 启动节点dave
# ./target/release/frontier-template-node purge-chain --base-path ./tmp/dave --chain local -y;\
# > ./tmp/dave.log;\
# pm2 start -n dave --log ./tmp/dave.log \
# ./target/release/frontier-template-node --\
#     --dave \
#     --chain local \
#     --port 1604 \
#     --rpc-port 1854 \
#     --ws-port 1804 \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --validator; 
# sleep 0.1s 

# # 启动节点eve
# ./target/release/frontier-template-node purge-chain --base-path ./tmp/eve --chain local -y;\
# > ./tmp/eve.log;\
# pm2 start -n eve --log ./tmp/eve.log \
# ./target/release/frontier-template-node --\
#     --eve \
#     --chain local \
#     --port 1605 \
#     --rpc-port 1855 \
#     --ws-port 1805 \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --validator; 
# sleep 0.1s 

# # 启动节点ferdie
# ./target/release/frontier-template-node purge-chain --base-path ./tmp/ferdie --chain local -y;\
# > ./tmp/ferdie.log;\
# pm2 start -n ferdie --log ./tmp/ferdie.log \
# ./target/release/frontier-template-node --\
#     --ferdie \
#     --chain local \
#     --port 1606 \
#     --rpc-port 1856 \
#     --ws-port 1806 \
#     --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
#     --validator; 
# sleep 0.1s 
