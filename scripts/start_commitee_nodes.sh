bootnode_peer_id="12D3KooWJTKdM3TkwGEjHcfEuWMHqAa9BjnmZJHX2mnCH4N35xaA"
spce_file="./tmp/RawAuraSpec.json"

# 启动节点C01
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C01 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C01 \
    --chain $spec_file \
    --port 33331 \
    --ws-port 1801 \
    --rpc-port 1851 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C01 &>./tmp/C01.log &\
sleep 0.1s

#启动节点C02
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C02 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C02 \
    --chain $spec_file \
    --port 33332 \
    --ws-port 1802 \
    --rpc-port 1852 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name C02 &>./tmp/C02.log &\
sleep 0.1s

#启动节点C03
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C03 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C03 \
    --chain $spec_file \
    --port 33333 \
    --ws-port 1803 \
    --rpc-port 1853 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name C03 &>./tmp/C03.log &\
sleep 0.1s

#启动节点C04
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C04 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C04 \
    --chain $spec_file \
    --port 33334 \
    --ws-port 1804 \
    --rpc-port 1854 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name C04 &>./tmp/C04.log &\
sleep 0.1s

#启动节点C05
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C05 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C05 \
    --chain $spec_file \
    --port 33335 \
    --ws-port 1805 \
    --rpc-port 1855 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name C05 &>./tmp/C05.log &\
sleep 0.1s

#启动节点C06
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C06 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C06 \
    --chain $spec_file \
    --port 33336 \
    --ws-port 1806 \
    --rpc-port 1856 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name C06 &>./tmp/C06.log &\
sleep 0.1s

#启动节点C07
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C07 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C07 \
    --chain $spec_file \
    --port 33337 \
    --ws-port 1807 \
    --rpc-port 1857 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name C07 &>./tmp/C07.log &\
sleep 0.1s

#启动节点C08
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C08 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C08 \
    --chain $spec_file \
    --port 33338 \
    --ws-port 1808 \
    --rpc-port 1858 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name C08 &>./tmp/C08.log &\
sleep 0.1s

#启动节点C09
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C09 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C09 \
    --chain $spec_file \
    --port 33339 \
    --ws-port 1809 \
    --rpc-port 1859 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name C09 &>./tmp/C09.log &\
sleep 0.1s

#启动节点C10
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C10 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C10 \
    --chain $spec_file \
    --port 33340 \
    --ws-port 1810 \
    --rpc-port 1860 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name C10 &>./tmp/C10.log &\
sleep 0.1s

#启动节点C11
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C11 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C11 \
    --chain $spec_file \
    --port 33341 \
    --ws-port 1811 \
    --rpc-port 1861 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name C11 &>./tmp/C11.log &\
sleep 0.1s

#启动节点C12
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C12 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C12 \
    --chain $spec_file \
    --port 33342 \
    --ws-port 1812 \
    --rpc-port 1862 \
    --validator \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name C12 &>./tmp/C12.log &\
sleep 0.1s

