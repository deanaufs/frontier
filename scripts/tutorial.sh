# 生成默认的配置模板
./target/debug/node-template build-spec --disable-default-bootnode --chain local > ./tmp/AuraSpec.json

# 根据模板，修改customSpec.json的aura, grandp内容，

# 产生启动时可用的Raw配置文件
./target/debug/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > ./tmp/RawAuraSpec.json

# 单独启动流程

# 启动节点1
# 按照生成的Raw配置文件启动node-template
./target/debug/node-template purge-chain --base-path ./tmp/node01 --chain local -y;\
./target/debug/node-template \
--base-path ./tmp/node01 \
--chain ./tmp/RawAuraSpec.json \
--port 30333 \
--ws-port 9945 \
--rpc-port 9933 \
--validator \
--rpc-methods Unsafe \
--name MyNode01

# 将aura 和 grandpa key添加到对应节点里面
./target/debug/node-template key insert \
--base-path ./tmp/node01 \
--chain ./tmp/RawAuraSpec.json \
--scheme Sr25519 \
--suri 0xe6c08bae89e39c5e4ed300b5824d7222f099a66121b24ec377e4b23b379ebada \
--password 123456 \
--key-type aura;\
./target/debug/node-template key insert \
--base-path ./tmp/node01 \
--chain ./tmp/RawAuraSpec.json \
--scheme Sr25519 \
--suri 0xe6c08bae89e39c5e4ed300b5824d7222f099a66121b24ec377e4b23b379ebada \
--password 123456 \
--key-type gran 

# 启动节点2
# 节点2的keystore配置
./target/debug/node-template purge-chain --base-path ./tmp/node02 --chain local -y;\
./target/debug/node-template \
--base-path ./tmp/node02 \
--chain ./tmp/AuraRawSpec.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--validator \
--rpc-methods Unsafe \
--name MyNode02 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRJTQudJu7RUXnoBNjdXjxU7K5PVpZevKCUXZH2b98fPA

# 将aura key添加到对应节点里面
./target/debug/node-template key insert \
--base-path ./tmp/node02 \
--chain ./tmp/RawAuraSpec.json \
--scheme Sr25519 \
--suri 0xf9debed82fdcfdff5c66ff1c6a3c3c0e44628b19f79d73ae377c45071344248e \
--password 123456 \
--key-type aura; \
./target/debug/node-template key insert \
--base-path ./tmp/node02 \
--chain ./tmp/RawAuraSpec.json \
--scheme Sr25519 \
--suri 0xf9debed82fdcfdff5c66ff1c6a3c3c0e44628b19f79d73ae377c45071344248e \
--password 123456 \
--key-type gran 

# 通过subkey 产生账户的key
./target/release/subkey generate --scheme Sr25519 --password 123456
# Secret phrase:       provide symptom praise empty hold celery lunar pet swamp egg fruit toss
#   Secret seed:       0xe6c08bae89e39c5e4ed300b5824d7222f099a66121b24ec377e4b23b379ebada
#   Public key (hex):  0x3eb829e3faff967d9adf283fe33b1184e887b38725798fbc9058f7e6e3d2f470
#   Account ID:        0x3eb829e3faff967d9adf283fe33b1184e887b38725798fbc9058f7e6e3d2f470
#   Public key (SS58): 5DUwXvm4ZML6hmVaMXHzJUBbmNVTQXsYRHcWhaMPKqYepfde
#   SS58 Address:      5DUwXvm4ZML6hmVaMXHzJUBbmNVTQXsYRHcWhaMPKqYepfde
./target/release/subkey inspect --password 123456 --scheme Ed25519 0xe6c08bae89e39c5e4ed300b5824d7222f099a66121b24ec377e4b23b379ebada
#   Secret seed:       0xe6c08bae89e39c5e4ed300b5824d7222f099a66121b24ec377e4b23b379ebada
#   Public key (hex):  0x5ba262fa04355f229290deebb000c443671e1ceba392c9159333ea522df0f8cc
#   Account ID:        0x5ba262fa04355f229290deebb000c443671e1ceba392c9159333ea522df0f8cc
#   Public key (SS58): 5E8rTG5AoXy3RXWLMLwCw3mrxQDkJHqAzNC7KXXvWBbEu5wg
#   SS58 Address:      5E8rTG5AoXy3RXWLMLwCw3mrxQDkJHqAzNC7KXXvWBbEu5wg

./target/release/subkey generate --scheme Sr25519 --password 123456
# Secret phrase:       cliff staff subject modify myself frost snack seminar thought rent dumb topic
#   Secret seed:       0xf9debed82fdcfdff5c66ff1c6a3c3c0e44628b19f79d73ae377c45071344248e
#   Public key (hex):  0x72e9385a5f6ea6a386a1aa45e4daa6266c188dc9415ea0240e87018883a0f335
#   Account ID:        0x72e9385a5f6ea6a386a1aa45e4daa6266c188dc9415ea0240e87018883a0f335
#   Public key (SS58): 5EfNba7RTK7R9v9QhLkU4XHiurzkKHyNqfXUdAn2qebb4BCn
#   SS58 Address:      5EfNba7RTK7R9v9QhLkU4XHiurzkKHyNqfXUdAn2qebb4BCn

./target/release/subkey inspect --password 123456 --scheme Ed25519 0xf9debed82fdcfdff5c66ff1c6a3c3c0e44628b19f79d73ae377c45071344248e
#   Secret seed:       0xf9debed82fdcfdff5c66ff1c6a3c3c0e44628b19f79d73ae377c45071344248e
#   Public key (hex):  0xc7b452df10d4b5c4be25b5293c6fd2babe36616f10d820eda3af9aee422af781
#   Account ID:        0xc7b452df10d4b5c4be25b5293c6fd2babe36616f10d820eda3af9aee422af781
#   Public key (SS58): 5GaYxFzaH8aQma35K9YRu7AW5dbVRZs5MpDjGt5AyMqAXHmX
#   SS58 Address:      5GaYxFzaH8aQma35K9YRu7AW5dbVRZs5MpDjGt5AyMqAXHmX


# 批量启动流程
# 1.先生成committee所需要的keys, committee的aura需要Sr25519类型的(出块使用)，grandpa需要Ed25519类型的(区块确认使用？)
    # ./target/release/subkey generate --scheme Sr25519 --password 123456
    # ./target/release/subkey inspect --password 123456 --scheme Ed25519 0xe6c08bae89e39c5e4ed300b5824d7222f099a66121b24ec377e4b23b379ebada
# 2.生成author所需要的keys，只需要Sr25519类型的
    # ./target/release/subkey generate --scheme Sr25519 --password 123456

# 3.使用脚本批量启动committee节点
# 4.使用脚本批量启动author节点
