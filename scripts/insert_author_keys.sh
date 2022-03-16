spec_file="./tmp/RawSpec.json"

# 节点1
# 将aura key添加到对应节点里面
./target/debug/frontier-template-node key insert \
    --base-path ./tmp/N01 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x3496dd931e8a69d69f0a9c30e4d8fbc028917d2b2a707789d8fdf9b74710e739 \
    --password 123456 \
    --key-type aura;\

# 节点2
# 将aura key添加到对应节点里面
./target/debug/frontier-template-node key insert \
    --base-path ./tmp/N02 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x0156a104d8df29d1e1fbc8fc51df487f0d1dfc2be7019e43d7955c8431c6863c \
    --password 123456 \
    --key-type aura;\

# 节点3
# 将aura key添加到对应节点里面
./target/debug/frontier-template-node key insert \
    --base-path ./tmp/N03 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x39d914d4a9170098e90031f3155984d6518360adc548104f1dfe5bd1c89284ee \
    --password 123456 \
    --key-type aura;\

# 节点4
# 将aura key添加到对应节点里面
./target/debug/frontier-template-node key insert \
    --base-path ./tmp/N04 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0xcc2f38bdc303cb3acdb36611dc5ca12719ed813d382601405eea6f3ddcdd5019 \
    --password 123456 \
    --key-type aura;\

# 节点5
# 将aura key添加到对应节点里面
./target/debug/frontier-template-node key insert \
    --base-path ./tmp/N05 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0xaa07d103d6f9885a17091aadcad5bd0ff21acbb107d354eaceafbe0cab4146ea \
    --password 123456 \
    --key-type aura;\

# 节点6
# 将aura key添加到对应节点里面
./target/debug/frontier-template-node key insert \
    --base-path ./tmp/N06 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0xebcc9a0574b36c72673db92e0d1801dec67ffaaed4e52818faf283a3b34bae3c \
    --password 123456 \
    --key-type aura;\

# 节点7
# 将aura key添加到对应节点里面
./target/debug/frontier-template-node key insert \
    --base-path ./tmp/N07 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x3b7b8fa964e9e4f171566c59a59b4b85b1655e350daf3edd6520c1ded60b75e0 \
    --password 123456 \
    --key-type aura;\

# 节点8
# 将aura key添加到对应节点里面
./target/debug/frontier-template-node key insert \
    --base-path ./tmp/N08 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x2f53e826c94bbf5b26c4596bff165c4b3cd974ae3590a325868295e68b4555ff \
    --password 123456 \
    --key-type aura;\

# 节点9
# 将aura key添加到对应节点里面
./target/debug/frontier-template-node key insert \
    --base-path ./tmp/N09 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x8af8efeed50d75c9684c3175c887bcaf700eab89ee28053bfad8af80487076a7 \
    --password 123456 \
    --key-type aura;\

# 节点10
# 将aura key添加到对应节点里面
./target/debug/frontier-template-node key insert \
    --base-path ./tmp/N10 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x31785588f6b8bf138d8dd9fdaa67a1d59d98db84a5d18d47514624e898aec913 \
    --password 123456 \
    --key-type aura;\

# 节点11
# 将aura key添加到对应节点里面
./target/debug/frontier-template-node key insert \
    --base-path ./tmp/N11 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x3a3df331300c030bd975ed1058ad9dd7016080188787a8ddbb286c7fff4cea03 \
    --password 123456 \
    --key-type aura;\

# 节点12
# 将aura key添加到对应节点里面
./target/debug/frontier-template-node key insert \
    --base-path ./tmp/N12 \
    --chain $spec_file \
    --scheme Sr25519 \
    --suri 0x6cd4e841b2da00416521348047b6739f81caeedf870a845849a5cd9a3e55096a \
    --password 123456 \
    --key-type aura;\

# # 节点13
# # 将aura key添加到对应节点里面
# ./target/debug/frontier-template-node key insert \
#     --base-path ./tmp/N13 \
#     --chain $spec_file \
#     --scheme Sr25519 \
#     --suri 0xbf7eccb25d501f9922bc6a76979869fc40923eea34d393c8fb7815f4e41d0aca \
#     --password 123456 \
#     --key-type aura;\

# # 节点14
# # 将aura key添加到对应节点里面
# ./target/debug/frontier-template-node key insert \
#     --base-path ./tmp/N14 \
#     --chain $spec_file \
#     --scheme Sr25519 \
#     --suri 0x5ba01e156f4ad65299142929c62419a8984295c4a5407906506d99a1c9430e1a \
#     --password 123456 \
#     --key-type aura;\

# # 节点15
# # 将aura key添加到对应节点里面
# ./target/debug/frontier-template-node key insert \
#     --base-path ./tmp/N15 \
#     --chain $spec_file \
#     --scheme Sr25519 \
#     --suri 0x07b7427313530a39429f98287dd747be2446b2f6b80098020e0fb8854658d47f \
#     --password 123456 \
#     --key-type aura;\

# # 节点16
# # 将aura key添加到对应节点里面
# ./target/debug/frontier-template-node key insert \
#     --base-path ./tmp/N16 \
#     --chain $spec_file \
#     --scheme Sr25519 \
#     --suri 0x5859b5e788c0df179e64ba64659ed1d2b1c6ce9df4754f1add92b17a68001df9 \
#     --password 123456 \
#     --key-type aura;\

# # 节点17
# # 将aura key添加到对应节点里面
# ./target/debug/frontier-template-node key insert \
#     --base-path ./tmp/N17 \
#     --chain $spec_file \
#     --scheme Sr25519 \
#     --suri 0xd1e64f74a90486aac3c5e90fb21309e06b43297cb19358ad5cb9692c1c821337 \
#     --password 123456 \
#     --key-type aura;\

# # 节点18
# # 将aura key添加到对应节点里面
# ./target/debug/frontier-template-node key insert \
#     --base-path ./tmp/N18 \
#     --chain $spec_file \
#     --scheme Sr25519 \
#     --suri 0x8a9e32b8f1692c58d88d7dab3d4740cf2eadd353bb0825a9a92571a8045eeb1b \
#     --password 123456 \
#     --key-type aura;\

# # 节点19
# # 将aura key添加到对应节点里面
# ./target/debug/frontier-template-node key insert \
#     --base-path ./tmp/N19 \
#     --chain $spec_file \
#     --scheme Sr25519 \
#     --suri 0x47254bf2df2e6a64c1d070af6d016e581fc2be1de1d7aec89e032ee8ac5b54fc \
#     --password 123456 \
#     --key-type aura;\

# # 节点20
# # 将aura key添加到对应节点里面
# ./target/debug/frontier-template-node key insert \
#     --base-path ./tmp/N20 \
#     --chain $spec_file \
#     --scheme Sr25519 \
#     --suri 0x31d6c2f7900c79d2cab3d2bc4afc01da2628a46aa1427145227cf9bd57b859e7 \
#     --password 123456 \
#     --key-type aura;\
