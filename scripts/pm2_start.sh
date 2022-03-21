# 启动节点alice
>./tmp/alice.log;\
./target/debug/frontier-template-node purge-chain --base-path ./tmp/alice --chain local -y;\
pm2 start -n alice --log ./tmp/alice.log \
"./target/debug/frontier-template-node \
    --base-path ./tmp/alice \
    --chain local \
    --alice \
    --port 1601 \
    --rpc-port 1851 \
    --ws-port 1801 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --validator \
    --rpc-cors all \
    --unsafe-rpc-external \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001";
sleep 0.1s

# 启动节点bob
>./tmp/bob.log;\
./target/debug/frontier-template-node purge-chain --base-path ./tmp/bob --chain local -y;\
pm2 start -n bob --log ./tmp/bob.log \
"./target/debug/frontier-template-node \
    --base-path ./tmp/bob \
    --chain local \
    --bob \
    --port 1602 \
    --rpc-port 1852 \
    --ws-port 1802 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --validator";
sleep 0.1s 

# 启动节点charlie
>./tmp/charlie.log;\
./target/debug/frontier-template-node purge-chain --base-path ./tmp/charlie --chain local -y;\
pm2 start -n charlie --log ./tmp/charlie.log \
"./target/debug/frontier-template-node \
    --base-path ./tmp/charlie \
    --chain local \
    --charlie \
    --port 1603 \
    --rpc-port 1853 \
    --ws-port 1803 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --validator";
sleep 0.1s 

# 启动节点dave
>./tmp/dave.log;\
./target/debug/frontier-template-node purge-chain --base-path ./tmp/dave --chain local -y;\
pm2 start -n dave --log ./tmp/dave.log \
"./target/debug/frontier-template-node \
    --base-path ./tmp/dave \
    --chain local \
    --dave \
    --port 1604 \
    --rpc-port 1854 \
    --ws-port 1804 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --validator";
sleep 0.1s 

# 启动节点eve
>./tmp/eve.log;\
./target/debug/frontier-template-node purge-chain --base-path ./tmp/eve --chain local -y;\
pm2 start -n eve --log ./tmp/eve.log \
"./target/debug/frontier-template-node \
    --base-path ./tmp/eve \
    --chain local \
    --eve \
    --port 1605 \
    --rpc-port 1855 \
    --ws-port 1805 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --validator";
sleep 0.1s 

# 启动节点ferdie
>./tmp/ferdie.log;\
./target/debug/frontier-template-node purge-chain --base-path ./tmp/ferdie --chain local -y;\
pm2 start -n ferdie --log ./tmp/ferdie.log \
"./target/debug/frontier-template-node \
    --base-path ./tmp/ferdie \
    --chain local \
    --ferdie \
    --port 1606 \
    --rpc-port 1856 \
    --ws-port 1806 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --validator";
sleep 0.1s 

