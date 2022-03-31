# 启动节点C01
./target/release/frontier-template-node purge-chain --base-path ./tmp/C01 --chain local -y;\
> ./tmp/C01.log;
pm2 start -n C01 --log ./tmp/C01.log ./target/release/frontier-template-node --\
    --name C01 \
    --port 1601 \
    --rpc-port 1851 \
    --ws-port 1801 \
    --validator \
    --rpc-cors all \
    --unsafe-rpc-external \
    --unsafe-ws-external \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --base-path ./tmp/C01 \
    --chain ./tmp/RawSpec.json;
sleep 0.1s