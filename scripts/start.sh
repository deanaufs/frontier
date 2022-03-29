bootnode_peer_id="12D3KooWJTKdM3TkwGEjHcfEuWMHqAa9BjnmZJHX2mnCH4N35xaA"
spec_file="./tmp/RawSpec.json"

# 启动节点C01
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C01 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C01 \
    --chain $spec_file \
    --public-addr ip4/221.121.151.89/tcp/1601\
    --port 1601 \
    --ws-port 1801 \
    --rpc-port 1851 \
    --validator \
    --rpc-cors all \
    --unsafe-rpc-external \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C01 &>./tmp/C01.log &\
sleep 0.1s

#启动节点C02
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C02 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C02 \
    --chain $spec_file \
    --public-addr ip4/221.121.151.89/tcp/1602\
    --port 1602 \
    --ws-port 1802 \
    --rpc-port 1852 \
    --validator \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C02 &>./tmp/C02.log &\
sleep 0.1s

#启动节点C03
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C03 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C03 \
    --chain $spec_file \
    --public-addr ip4/221.121.151.89/tcp/1603\
    --port 1603 \
    --ws-port 1803 \
    --rpc-port 1853 \
    --validator \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C03 &>./tmp/C03.log &\
sleep 0.1s

#启动节点C04
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C04 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C04 \
    --chain $spec_file \
    --public-addr ip4/221.121.151.89/tcp/1604\
    --port 1604 \
    --ws-port 1804 \
    --rpc-port 1854 \
    --validator \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C04 &>./tmp/C04.log &\
sleep 0.1s

#启动节点C05
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C05 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C05 \
    --chain $spec_file \
    --public-addr ip4/221.121.151.89/tcp/1605\
    --port 1605 \
    --ws-port 1805 \
    --rpc-port 1855 \
    --validator \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C05 &>./tmp/C05.log &\
sleep 0.1s

#启动节点C06
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C06 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C06 \
    --chain $spec_file \
    --public-addr ip4/221.121.151.89/tcp/1606\
    --port 1606 \
    --ws-port 1806 \
    --rpc-port 1856 \
    --validator \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C06 &>./tmp/C06.log &\
sleep 0.1s

#启动节点C07
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C07 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C07 \
    --chain $spec_file \
    --public-addr ip4/221.121.151.89/tcp/1607\
    --port 1607 \
    --ws-port 1807 \
    --rpc-port 1857 \
    --validator \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C07 &>./tmp/C07.log &\
sleep 0.1s

#启动节点C08
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C08 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C08 \
    --chain $spec_file \
    --public-addr ip4/221.121.151.89/tcp/1608\
    --port 1608 \
    --ws-port 1808 \
    --rpc-port 1858 \
    --validator \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C08 &>./tmp/C08.log &\
sleep 0.1s

#启动节点C09
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C09 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C09 \
    --chain $spec_file \
    --public-addr ip4/221.121.151.89/tcp/1609\
    --port 1609 \
    --ws-port 1809 \
    --rpc-port 1859 \
    --validator \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C09 &>./tmp/C09.log &\
sleep 0.1s

#启动节点C10
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C10 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C10 \
    --chain $spec_file \
    --public-addr ip4/221.121.151.89/tcp/1610\
    --port 1610 \
    --ws-port 1810 \
    --rpc-port 1860 \
    --validator \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C10 &>./tmp/C10.log &\
sleep 0.1s

#启动节点C11
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C11 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C11 \
    --chain $spec_file \
    --public-addr ip4/221.121.151.89/tcp/1611\
    --port 1611 \
    --ws-port 1811 \
    --rpc-port 1861 \
    --validator \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C11 &>./tmp/C11.log &\
sleep 0.1s

#启动节点C12
./target/debug/frontier-template-node purge-chain --base-path ./tmp/C12 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/C12 \
    --chain $spec_file \
    --public-addr ip4/221.121.151.89/tcp/1612\
    --port 1612 \
    --ws-port 1812 \
    --rpc-port 1862 \
    --validator \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name C12 &>./tmp/C12.log &\
sleep 0.1s

bootnode_peer_id="12D3KooWJTKdM3TkwGEjHcfEuWMHqAa9BjnmZJHX2mnCH4N35xaA"
spec_file="./tmp/RawAuraSpec.json"

#启动节点N01
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N01 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N01 \
    --public-addr ip4/221.121.151.89/tcp/1631\
    --chain $spec_file \
    --port 1631 \
    --ws-port 1831 \
    --rpc-port 1864 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N01 &>./tmp/N01.log &\
sleep 0.1s

#启动节点N02
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N02 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N02 \
    --public-addr ip4/221.121.151.89/tcp/1632\
    --chain $spec_file \
    --port 1632 \
    --ws-port 1832 \
    --rpc-port 1865 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N02 &>./tmp/N02.log &\
sleep 0.1s

#启动节点N03
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N03 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N03 \
    --public-addr ip4/221.121.151.89/tcp/1633\
    --chain $spec_file \
    --port 1633 \
    --ws-port 1833 \
    --rpc-port 1866 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N03 &>./tmp/N03.log &\
sleep 0.1s

#启动节点N04
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N04 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N04 \
    --public-addr ip4/221.121.151.89/tcp/1634\
    --chain $spec_file \
    --port 1634 \
    --ws-port 1834 \
    --rpc-port 1867 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N04 &>./tmp/N04.log &\
sleep 0.1s

#启动节点N05
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N05 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N05 \
    --public-addr ip4/221.121.151.89/tcp/1635\
    --chain $spec_file \
    --port 1635 \
    --ws-port 1835 \
    --rpc-port 1868 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N05 &>./tmp/N05.log &\
sleep 0.1s

#启动节点N06
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N06 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N06 \
    --public-addr ip4/221.121.151.89/tcp/1636\
    --chain $spec_file \
    --port 1636 \
    --ws-port 1836 \
    --rpc-port 1869 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N06 &>./tmp/N06.log &\
sleep 0.1s

#启动节点N07
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N07 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N07 \
    --public-addr ip4/221.121.151.89/tcp/1637\
    --chain $spec_file \
    --port 1637 \
    --ws-port 1837 \
    --rpc-port 1870 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N07 &>./tmp/N07.log &\
sleep 0.1s

#启动节点N08
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N08 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N08 \
    --public-addr ip4/221.121.151.89/tcp/1638\
    --chain $spec_file \
    --port 1638 \
    --ws-port 1838 \
    --rpc-port 1871 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N08 &>./tmp/N08.log &\
sleep 0.1s

#启动节点N09
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N09 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N09 \
    --public-addr ip4/221.121.151.89/tcp/1639\
    --chain $spec_file \
    --port 1639 \
    --ws-port 1839 \
    --rpc-port 1872 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N09 &>./tmp/N09.log &\
sleep 0.1s

#启动节点N10
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N10 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N10 \
    --public-addr ip4/221.121.151.89/tcp/1640\
    --chain $spec_file \
    --port 1640 \
    --ws-port 1840 \
    --rpc-port 1873 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N10 &>./tmp/N10.log &\
sleep 0.1s

#启动节点N11
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N11 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N11 \
    --public-addr ip4/221.121.151.89/tcp/1641\
    --chain $spec_file \
    --port 1641 \
    --ws-port 1841 \
    --rpc-port 1874 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N11 &>./tmp/N11.log &\
sleep 0.1s

#启动节点N12
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N12 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N12 \
    --public-addr ip4/221.121.151.89/tcp/1642\
    --chain $spec_file \
    --port 1642 \
    --ws-port 1842 \
    --rpc-port 1875 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N12 &>./tmp/N12.log &\
sleep 0.1s

#启动节点N13
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N13 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N13 \
    --public-addr ip4/221.121.151.89/tcp/1643\
    --chain $spec_file \
    --port 1643 \
    --ws-port 1843 \
    --rpc-port 1876 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N13 &>./tmp/N13.log &\
sleep 0.1s

#启动节点N14
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N14 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N14 \
    --public-addr ip4/221.121.151.89/tcp/1644\
    --chain $spec_file \
    --port 1644 \
    --ws-port 1844 \
    --rpc-port 1877 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N14 &>./tmp/N14.log &\
sleep 0.1s

#启动节点N15
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N15 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N15 \
    --public-addr ip4/221.121.151.89/tcp/1645\
    --chain $spec_file \
    --port 1645 \
    --ws-port 1845 \
    --rpc-port 1878 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N15 &>./tmp/N15.log &\
sleep 0.1s

#启动节点N16
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N16 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N16 \
    --public-addr ip4/221.121.151.89/tcp/1646\
    --chain $spec_file \
    --port 1646 \
    --ws-port 1846 \
    --rpc-port 1879 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N16 &>./tmp/N16.log &\
sleep 0.1s

#启动节点N17
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N17 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N17 \
    --public-addr ip4/221.121.151.89/tcp/1647\
    --chain $spec_file \
    --port 1647 \
    --ws-port 1847 \
    --rpc-port 1880 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N17 &>./tmp/N17.log &\
sleep 0.1s

#启动节点N18
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N18 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N18 \
    --public-addr ip4/221.121.151.89/tcp/1648\
    --chain $spec_file \
    --port 1648 \
    --ws-port 1848 \
    --rpc-port 1881 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N18 &>./tmp/N18.log &\
sleep 0.1s

#启动节点N19
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N19 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N19 \
    --public-addr ip4/221.121.151.89/tcp/1649\
    --chain $spec_file \
    --port 1649 \
    --ws-port 1849 \
    --rpc-port 1882 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N19 &>./tmp/N19.log &\
sleep 0.1s

#启动节点N20
./target/debug/frontier-template-node purge-chain --base-path ./tmp/N20 --chain local -y;\
./target/debug/frontier-template-node \
    --base-path ./tmp/N20 \
    --public-addr ip4/221.121.151.89/tcp/1650\
    --chain $spec_file \
    --port 1650 \
    --ws-port 1850 \
    --rpc-port 1883 \
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
    --name N20 &>./tmp/N20.log &\
sleep 0.1s
