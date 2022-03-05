spec_file="./tmp/RawSpec.json"

# 节点1
# 将aura 和 grandpa key添加到对应节点里面
./target/debug/node-template key insert \
    --base-path ./tmp/C01 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x019216df060179fab7ae75981f7c875fb75eb47e6ff89bcf112d8e3b8ccefa75 \
    --password 123456 \
    --key-type aura;\
./target/debug/node-template key insert \
    --base-path ./tmp/C01 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x019216df060179fab7ae75981f7c875fb75eb47e6ff89bcf112d8e3b8ccefa75 \
    --password 123456 \
    --key-type gran 

# 节点2
# 将aura 和 grandpa key添加到对应节点里面
./target/debug/node-template key insert \
    --base-path ./tmp/C02 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0xc07b5fa178d06e545049f117d94633e576271202b6e03665d935b9d89c5055c6 \
    --password 123456 \
    --key-type aura;\
./target/debug/node-template key insert \
    --base-path ./tmp/C02 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0xc07b5fa178d06e545049f117d94633e576271202b6e03665d935b9d89c5055c6 \
    --password 123456 \
    --key-type gran 

# 节点3
# 将aura 和 grandpa key添加到对应节点里面
./target/debug/node-template key insert \
    --base-path ./tmp/C03 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0xaf905333522e44c57ece667d7c129b0431a81bd8bc336be74d4e7b17dc56bf76 \
    --password 123456 \
    --key-type aura;\
./target/debug/node-template key insert \
    --base-path ./tmp/C03 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0xaf905333522e44c57ece667d7c129b0431a81bd8bc336be74d4e7b17dc56bf76 \
    --password 123456 \
    --key-type gran 

# 节点4
# 将aura 和 grandpa key添加到对应节点里面
./target/debug/node-template key insert \
    --base-path ./tmp/C04 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x0a7c1ab47570a2ddb6d40b8bffa1d64c54b38adfb48590a25f7dcb4d927034fd \
    --password 123456 \
    --key-type aura;\
./target/debug/node-template key insert \
    --base-path ./tmp/C04 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x0a7c1ab47570a2ddb6d40b8bffa1d64c54b38adfb48590a25f7dcb4d927034fd \
    --password 123456 \
    --key-type gran 

# 节点5
# 将aura 和 grandpa key添加到对应节点里面
./target/debug/node-template key insert \
    --base-path ./tmp/C05 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x02340296091e3243b96824a99a630381cd0bf16326d90a7ff3379a71d4a72fb1 \
    --password 123456 \
    --key-type aura;\
./target/debug/node-template key insert \
    --base-path ./tmp/C05 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x02340296091e3243b96824a99a630381cd0bf16326d90a7ff3379a71d4a72fb1 \
    --password 123456 \
    --key-type gran 

# 节点6
# 将aura 和 grandpa key添加到对应节点里面
./target/debug/node-template key insert \
    --base-path ./tmp/C06 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x78589be680b825bbc829f546db4c2856d8d267293344cc4a62ecd4d281908ac3 \
    --password 123456 \
    --key-type aura;\
./target/debug/node-template key insert \
    --base-path ./tmp/C06 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x78589be680b825bbc829f546db4c2856d8d267293344cc4a62ecd4d281908ac3 \
    --password 123456 \
    --key-type gran 

# 节点7
# 将aura 和 grandpa key添加到对应节点里面
./target/debug/node-template key insert \
    --base-path ./tmp/C07 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x89efbc31ab5997e5a9546aa5744e072b8ac539ef92755b13af5e4f413b409b7d \
    --password 123456 \
    --key-type aura;\
./target/debug/node-template key insert \
    --base-path ./tmp/C07 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x89efbc31ab5997e5a9546aa5744e072b8ac539ef92755b13af5e4f413b409b7d \
    --password 123456 \
    --key-type gran 

# 节点8
# 将aura 和 grandpa key添加到对应节点里面
./target/debug/node-template key insert \
    --base-path ./tmp/C08 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0xbfef9d899e39f83ec789eb552e4db5dfa8fb72a6b6dbedbbaa7592681fdfca4a \
    --password 123456 \
    --key-type aura;\
./target/debug/node-template key insert \
    --base-path ./tmp/C08 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0xbfef9d899e39f83ec789eb552e4db5dfa8fb72a6b6dbedbbaa7592681fdfca4a \
    --password 123456 \
    --key-type gran 

# # 节点9
# # 将aura 和 grandpa key添加到对应节点里面
# ./target/debug/node-template key insert \
#     --base-path ./tmp/C09 \
#     --chain $spec_file \
#     --scheme Sr25519 \
#     --suri 0x3ade619f2eb08ddea422358ee2e16e9e90726513bfb7d63f040d6e1ec54e3849 \
#     --password 123456 \
#     --key-type aura;\
# ./target/debug/node-template key insert \
#     --base-path ./tmp/C09 \
#     --chain $spec_file \
#     --scheme Sr25519 \
#     --suri 0x3ade619f2eb08ddea422358ee2e16e9e90726513bfb7d63f040d6e1ec54e3849 \
#     --password 123456 \
#     --key-type gran 

# # 节点10
# # 将aura 和 grandpa key添加到对应节点里面
# ./target/debug/node-template key insert \
#     --base-path ./tmp/C10 \
#     --chain $spec_file \
#     --scheme Sr25519 \
#     --suri 0x856a4b72587ae954b2cd4520101cfb5c99b9c7e7d8a00c641473201178d3679f \
#     --password 123456 \
#     --key-type aura;\
# ./target/debug/node-template key insert \
#     --base-path ./tmp/C10 \
#     --chain $spec_file \
#     --scheme Sr25519 \
#     --suri 0x856a4b72587ae954b2cd4520101cfb5c99b9c7e7d8a00c641473201178d3679f \
#     --password 123456 \
#     --key-type gran 
