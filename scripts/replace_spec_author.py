import json
import sys

author_list =  [
    "5HpxdSjV2j5uedrXjSRjctJXBPwaE1h22o2jqcMi4cT7MpNA",
    "5D2UJ9FSUPBHbY1QvAbNkoMvaVckq6k2sGS33gmVGd5dxYQX",
    "5EX4A9u1zm7DZKRSnr4FhGcU1gsySAqhfvyndnkj7KroWsq9",
    "5FEvjo9Kt8XZ3AaGKu9ie4dCKeZs1b3HZNEM2t78i8t6hUnQ",
    "5GU4Tp8xDbFv7HCQoNcTQeLz5LTMLozz1y5dJ7pbweUhtCj7",
    # "5GHR8GWUQc26zfKwJnpSMiguxQ15wo19S6YANEQMvkYSsv8C",
    # "5CzMVFENrd6GvrQG9Ee4P7HTmqG4DV8pVXeCXfuDXYvW6xDf",
    # "5HbernvcXwMbh7gMwLq8Mu8mcmnGdCvq6243nB8KNJqFhRnu"
]
grandpa_list = [
    "5HDwJGQCGkWdKAV2VbJA8S4JM8AQZkJxe3oHj2epgGytqyV2",
    "5EyPuBJM2MvBRi1LWUWV7MGVKRWqEPXL4oryVHYKtAdG7RSj",
    "5GRUhGV6hAQt5cErVMExbmQ2eVo7GcBtc6Zoc5ty9xcyeHMZ",
    "5Drd7gm2kyGHafAk8QvRsX8EDNJekE29x9vNKKuKTg33avGz",
    "5Dh49fVddbdUVvaC4uJAr6R8SKbzSq8LB8SNigk4zWJVa5sy",
    # "5DtM68dDEhM9uSVT1zcuocdUD6rF36LNNUB6mfBX21ni6jzu",
    # "5FUseKbeL7e28d1tV3PV83mrkdGLPC6JdNQPbdpSJPL4bPAM",
    # "5H9y47kuQJmXuwwXvjUYRQxDNkozyFbHVcyZ5nQuqp9Saqye",
]

# try:
input_file = sys.argv[1]
output_file = sys.argv[2]
# print(sys.argv)

with open(input_file) as f:
# with open("./tmp/Spec.json") as f:
    msg = json.load(f)
    # print(msg.keys())
    vote_authors = author_list
    msg["genesis"]["runtime"]["voteElection"]["authorities"]=vote_authors
    # print(msg["genesis"]["runtime"]["voteElection"]["authorities"])

    grandpa_authors = []
    for i in grandpa_list:
        grandpa_authors.append([i, 1])
    
    # print(grandpa_authors)
    msg["genesis"]["runtime"]["grandpa"]["authorities"]=grandpa_authors

    with open(output_file, "w") as nf:
    # with open("./tmp/TempSpec.json", "w") as nf:
        json.dump(msg, nf, indent=2)
        print("Build RawSpec Success")


