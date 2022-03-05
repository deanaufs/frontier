bootnode_peer_id="12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"
spec_file="./tmp/RawAuraSpec.json"

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

bootnode_peer_id="12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"
spce_file="./tmp/RawAuraSpec.json"

#启动节点N01
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N01 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N01 \
    --chain $spec_file \
    --port 33361 \
    --ws-port 1831 \
    --rpc-port 1864 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N01 &>./tmp/N01.log &\
sleep 0.1s

#启动节点N02
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N02 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N02 \
    --chain $spec_file \
    --port 33362 \
    --ws-port 1832 \
    --rpc-port 1865 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N02 &>./tmp/N02.log &\
sleep 0.1s

#启动节点N03
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N03 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N03 \
    --chain $spec_file \
    --port 33363 \
    --ws-port 1833 \
    --rpc-port 1866 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N03 &>./tmp/N03.log &\
sleep 0.1s

#启动节点N04
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N04 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N04 \
    --chain $spec_file \
    --port 33364 \
    --ws-port 1834 \
    --rpc-port 1867 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N04 &>./tmp/N04.log &\
sleep 0.1s

#启动节点N05
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N05 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N05 \
    --chain $spec_file \
    --port 33365 \
    --ws-port 1835 \
    --rpc-port 1868 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N05 &>./tmp/N05.log &\
sleep 0.1s

#启动节点N06
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N06 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N06 \
    --chain $spec_file \
    --port 33366 \
    --ws-port 1836 \
    --rpc-port 1869 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N06 &>./tmp/N06.log &\
sleep 0.1s

#启动节点N07
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N07 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N07 \
    --chain $spec_file \
    --port 33367 \
    --ws-port 1837 \
    --rpc-port 1870 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N07 &>./tmp/N07.log &\
sleep 0.1s

#启动节点N08
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N08 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N08 \
    --chain $spec_file \
    --port 33368 \
    --ws-port 1838 \
    --rpc-port 1871 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N08 &>./tmp/N08.log &\
sleep 0.1s

#启动节点N09
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N09 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N09 \
    --chain $spec_file \
    --port 33369 \
    --ws-port 1839 \
    --rpc-port 1872 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N09 &>./tmp/N09.log &\
sleep 0.1s

#启动节点N10
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N10 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N10 \
    --chain $spec_file \
    --port 33370 \
    --ws-port 1840 \
    --rpc-port 1873 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N10 &>./tmp/N10.log &\
sleep 0.1s

#启动节点N11
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N11 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N11 \
    --chain $spec_file \
    --port 33371 \
    --ws-port 1841 \
    --rpc-port 1874 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N11 &>./tmp/N11.log &\
sleep 0.1s

#启动节点N12
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N12 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N12 \
    --chain $spec_file \
    --port 33372 \
    --ws-port 1842 \
    --rpc-port 1875 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N12 &>./tmp/N12.log &\
sleep 0.1s

#启动节点N13
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N13 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N13 \
    --chain $spec_file \
    --port 33373 \
    --ws-port 1843 \
    --rpc-port 1876 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N13 &>./tmp/N13.log &\
sleep 0.1s

#启动节点N14
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N14 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N14 \
    --chain $spec_file \
    --port 33374 \
    --ws-port 1844 \
    --rpc-port 1877 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N14 &>./tmp/N14.log &\
sleep 0.1s

#启动节点N15
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N15 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N15 \
    --chain $spec_file \
    --port 33375 \
    --ws-port 1845 \
    --rpc-port 1878 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N15 &>./tmp/N15.log &\
sleep 0.1s

#启动节点N16
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N16 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N16 \
    --chain $spec_file \
    --port 33376 \
    --ws-port 1846 \
    --rpc-port 1879 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N16 &>./tmp/N16.log &\
sleep 0.1s

#启动节点N17
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N17 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N17 \
    --chain $spec_file \
    --port 33377 \
    --ws-port 1847 \
    --rpc-port 1880 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N17 &>./tmp/N17.log &\
sleep 0.1s

#启动节点N18
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N18 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N18 \
    --chain $spec_file \
    --port 33378 \
    --ws-port 1848 \
    --rpc-port 1881 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N18 &>./tmp/N18.log &\
sleep 0.1s

#启动节点N19
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N19 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N19 \
    --chain $spec_file \
    --port 33379 \
    --ws-port 1849 \
    --rpc-port 1882 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N19 &>./tmp/N19.log &\
sleep 0.1s

#启动节点N20
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N20 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N20 \
    --chain $spec_file \
    --port 33380 \
    --ws-port 1850 \
    --rpc-port 1883 \
    --rpc-methods Unsafe \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --bootnodes /ip4/127.0.0.1/tcp/33331/p2p/$bootnode_peer_id \
    --name N20 &>./tmp/N20.log &\
sleep 0.1s

