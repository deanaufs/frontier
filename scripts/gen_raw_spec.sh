# 生成默认的配置模板
./target/debug/frontier-template-node build-spec --disable-default-bootnode --chain local > ./tmp/Spec.json

# 根据模板，修改customSpec.json的aura, grandp内容，
python3 ./scripts/replace_spec_author.py ./tmp/Spec.json ./tmp/TempSpec.json

# 产生启动时可用的Raw配置文件
./target/debug/frontier-template-node build-spec --chain=./tmp/TempSpec.json --raw --disable-default-bootnode > ./tmp/RawSpec.json
