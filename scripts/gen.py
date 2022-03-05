PORT, WS_PORT, RPC_PORT = 33331, 1801, 1851
committee_count = 12
author_count = 20
app_name = "frontier-template-node"
spec_file_config = "spec_file=\"./tmp/RawAuraSpec.json\""

boot_node_str = """# 启动节点C01
./target/debug/{3} purge-chain --base-path ./tmp/C01 --chain local -y;\\
./target/debug/{3} \\
    --base-path ./tmp/C01 \\
    --chain $spec_file \\
    --port {0} \\
    --ws-port {1} \\
    --rpc-port {2} \\
    --validator \\
    --rpc-methods Unsafe \\
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \\
    --name C01 &>./tmp/C01.log &\\
sleep 0.1s
""".format(PORT, WS_PORT, RPC_PORT, app_name)

committee_node_str = """#启动节点{0}
./target/debug/{1} purge-chain --base-path ./tmp/{0} --chain local -y;\\
./target/debug/{1} \\
    --base-path ./tmp/{0} \\
    --chain $spec_file \\
    --port {2} \\
    --ws-port {3} \\
    --rpc-port {4} \\
    --validator \\
    --rpc-methods Unsafe \\
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \\
    --bootnodes /ip4/127.0.0.1/tcp/{5}/p2p/$bootnode_peer_id \\
    --name {0} &>./tmp/{0}.log &\\
sleep 0.1s
"""

author_node_str = """#启动节点{0}
./target/debug/{1} purge-chain --base-path ./tmp/{0} --chain local -y;\\
./target/debug/{1} \\
    --base-path ./tmp/{0} \\
    --chain $spec_file \\
    --port {2} \\
    --ws-port {3} \\
    --rpc-port {4} \\
    --rpc-methods Unsafe \\
    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \\
    --bootnodes /ip4/127.0.0.1/tcp/{5}/p2p/$bootnode_peer_id \\
    --name {0} &>./tmp/{0}.log &\\
sleep 0.1s
"""

print("bootnode_peer_id=\"12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp\"")
print(spec_file_config)
# print("spec_file=\"{}\"".format(spec_file))
print()
for i in range(0, committee_count):
    if i == 0:
        print(boot_node_str)
    else:
        node_name = "C{:02d}".format(i+1)
        port = PORT + i
        ws_port = WS_PORT + i
        rpc_port = RPC_PORT + i

        print(committee_node_str.format(node_name, app_name, port, ws_port, rpc_port, PORT))

# print(fmt_str)
print("bootnode_peer_id=\"12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp\"")
print(spec_file_config)
print()
for i in range(0, author_count):

    node_name = "N{:02d}".format(i+1)
    port = PORT + int((committee_count+20)/10)*10 + i
    ws_port = WS_PORT + int((committee_count+20)/10)*10 + i
    rpc_port = RPC_PORT + int((committee_count+20)/10) + 10+ i

    print(author_node_str.format(node_name, app_name, port, ws_port, rpc_port, PORT))