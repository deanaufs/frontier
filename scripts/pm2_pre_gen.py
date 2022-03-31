
bootnode_cmd = """# 启动节点{0}
>./tmp/{0}.log;\\
./target/{4}/frontier-template-node purge-chain --base-path ./tmp/{0} --chain local -y;\\
pm2 start -n {0} --log ./tmp/{0}.log \\
"./target/{4}/frontier-template-node \\
    --base-path ./tmp/{0} \\
    --chain local \\
    --{0} \\
    --port {1} \\
    --rpc-port {2} \\
    --ws-port {3} \\
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \\
    --validator \\
    --rpc-cors all \\
    --unsafe-rpc-external \\
    --unsafe-ws-external \\
    --node-key 0000000000000000000000000000000000000000000000000000000000000001";
sleep 0.1s
"""

node_cmd = """# 启动节点{0}
>./tmp/{0}.log;\\
./target/{4}/frontier-template-node purge-chain --base-path ./tmp/{0} --chain local -y;\\
pm2 start -n {0} --log ./tmp/{0}.log \\
"./target/{4}/frontier-template-node \\
    --base-path ./tmp/{0} \\
    --chain local \\
    --{0} \\
    --port {1} \\
    --rpc-port {2} \\
    --ws-port {3} \\
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \\
    --validator"; 
sleep 0.1s 
"""

PORT_NO, RPC_PORT_NO, WS_PORT_NO = 1601, 1851, 1801
account_list = ["alice", "bob", "charlie", "dave", "eve", "ferdie"]
mode="release"

# for (i,  account) in account_list
port, rpc_port, ws_port = PORT_NO, RPC_PORT_NO, WS_PORT_NO
for (i, a) in enumerate(account_list):
    if i == 0:
        print(bootnode_cmd.format(a, port+i, rpc_port+i, ws_port+i, mode))
    else:
        print(node_cmd.format(a, port+i, rpc_port+i, ws_port+i, mode))