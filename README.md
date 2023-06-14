# Web3-Dex-Sei

## Compile

Confirm that the prerequisites listed in the CosmWasm docs are met, and then run the following

```
cargo wasm
```

Before uploading, use the rust-optimizer to minimize the size of the binary that will be uploaded

```
docker run  -v "$(pwd):/code" \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11
```

## optimized test

```
docker run  -v "$(pwd):/code" \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/rust-optimizer-arm64:0.12.11
```

## Deploy

Store a compiled wasm binary to the sei network:


```
export ACCOUNT_ADDRESS=sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak
export ACCOUNT_NAME=shaneson
export CONTRACT_WASM_BINARY=./artifacts/wasm_dexrouter-aarch64.wasm
export ENDPOINT=https://sei-testnet-rpc.polkachu.com
export CHAINID=atlantic-2
```

```
seid tx wasm store $CONTRACT_WASM_BINARY -y --from=$ACCOUNT_NAME --node $ENDPOINT --chain-id=$CHAINID --gas=1202313 --fees=120232usei --broadcast-mode=block

.....

(base) ➜  wasm-dexrouter git:(main) ✗ seid tx wasm store $CONTRACT_WASM_BINARY -y --from=$ACCOUNT_NAME --node $ENDPOINT --chain-id=$CHAINID --gas=1202313 --fees=120232usei --broadcast-mode=block

code: 0
codespace: ""
data: 0A250A1E2F636F736D7761736D2E7761736D2E76312E4D736753746F7265436F6465120308F508
events:
- attributes:
  - index: true
    key: c3BlbmRlcg==
    value: c2VpMXE3OWtrend6bXdlbnp6ZGFlNDc0ZXRncXM1Y2pxeGxzaDRjcGFr
  - index: true
    key: YW1vdW50
    value: MTIwMjMydXNlaQ==
  type: coin_spent
- attributes:
  - index: true
    key: ZmVl
    value: MTIwMjMydXNlaQ==
  - index: true
    key: ZmVlX3BheWVy
    value: c2VpMXE3OWtrend6bXdlbnp6ZGFlNDc0ZXRncXM1Y2pxeGxzaDRjcGFr
  type: tx
- attributes:
  - index: true
    key: YWNjX3NlcQ==
    value: c2VpMXE3OWtrend6bXdlbnp6ZGFlNDc0ZXRncXM1Y2pxeGxzaDRjcGFrLzU=
  type: tx
- attributes:
  - index: true
    key: c2lnbmF0dXJl
    value: MzRVUG5EckZENGllOWd4Yk9tWEZtdEpESlVFM2pycHpuSXZJeEtLZll3TkhDbHp4T0VSWDNiZW9PNVVwU1BZd2lhcUJDWVVjV2p4a2NLcUs5UGczTGc9PQ==
  type: tx
- attributes:
  - index: true
    key: YWN0aW9u
    value: L2Nvc213YXNtLndhc20udjEuTXNnU3RvcmVDb2Rl
  type: message
- attributes:
  - index: true
    key: bW9kdWxl
    value: d2FzbQ==
  - index: true
    key: c2VuZGVy
    value: c2VpMXE3OWtrend6bXdlbnp6ZGFlNDc0ZXRncXM1Y2pxeGxzaDRjcGFr
  type: message
- attributes:
  - index: true
    key: Y29kZV9pZA==
    value: MTE0MQ==
  type: store_code
gas_used: "1110680"
gas_wanted: "1202313"
height: "15184146"
info: ""
logs:
- events:
  - attributes:
    - key: action
      value: /cosmwasm.wasm.v1.MsgStoreCode
    - key: module
      value: wasm
    - key: sender
      value: sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak
    type: message
  - attributes:
    - key: code_id
      value: "1141"
    type: store_code
  log: ""
  msg_index: 0
raw_log: '[{"events":[{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgStoreCode"},{"key":"module","value":"wasm"},{"key":"sender","value":"sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak"}]},{"type":"store_code","attributes":[{"key":"code_id","value":"1141"}]}]}]'
timestamp: ""
tx: null
txhash: A2A44937724C445975981E34480E8CE2CBC21A345322238DA3316FF148437C1F


```

Once your proposal is stored to the testnet, you can instantiate your contract:

```
export CONTRACT_ID=1141
export LABEL="wasm-dexrouter"
seid tx wasm instantiate $CONTRACT_ID '{"count": 0}' --chain-id $CHAINID --from $ACCOUNT_NAME --gas=4000000 --fees=50000usei --broadcast-mode=block --label $LABEL --admin $ACCOUNT_ADDRESS --node $ENDPOINT
```

Note that the '{}' part is the parameters you pass to instantiate the contract. In this example, the contract takes no parameter so '{}' suffices. For any real world contracts, their instantiation parameters would likely be non-empty. 
You should get a response like the following:

```
(base) ➜  wasm-dexrouter git:(main) ✗ seid tx wasm instantiate $CONTRACT_ID '{"count": 0}' --chain-id $CHAINID --from $ACCOUNT_NAME --gas=4000000 --fees=50000usei --broadcast-mode=block --label $LABEL --admin $ACCOUNT_ADDRESS --node $ENDPOINT

{"body":{"messages":[{"@type":"/cosmwasm.wasm.v1.MsgInstantiateContract","sender":"sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak","admin":"sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak","code_id":"1141","label":"wasm-dexrouter","msg":{"count":0},"funds":[]}],"memo":"","timeout_height":"0","extension_options":[],"non_critical_extension_options":[]},"auth_info":{"signer_infos":[],"fee":{"amount":[{"denom":"usei","amount":"50000"}],"gas_limit":"4000000","payer":"","granter":""}},"signatures":[]}

confirm transaction before signing and broadcasting [y/N]: y
code: 0
codespace: ""
data: 0A6C0A282F636F736D7761736D2E7761736D2E76312E4D7367496E7374616E7469617465436F6E747261637412400A3E736569316E6C32646368733667703874796478743077733964323534306D3279727633756D7A7561336761617A7567686439733579776373713875676168
events:
- attributes:
  - index: true
    key: c3BlbmRlcg==
    value: c2VpMXE3OWtrend6bXdlbnp6ZGFlNDc0ZXRncXM1Y2pxeGxzaDRjcGFr
  - index: true
    key: YW1vdW50
    value: NTAwMDB1c2Vp
  type: coin_spent
- attributes:
  - index: true
    key: ZmVl
    value: NTAwMDB1c2Vp
  - index: true
    key: ZmVlX3BheWVy
    value: c2VpMXE3OWtrend6bXdlbnp6ZGFlNDc0ZXRncXM1Y2pxeGxzaDRjcGFr
  type: tx
- attributes:
  - index: true
    key: YWNjX3NlcQ==
    value: c2VpMXE3OWtrend6bXdlbnp6ZGFlNDc0ZXRncXM1Y2pxeGxzaDRjcGFrLzc=
  type: tx
- attributes:
  - index: true
    key: c2lnbmF0dXJl
    value: T1BpQUZVU3l3ckRzM3pVVll6Vnd5Q3ZHSWwzS0tiSng0cDhHbU8xWWNLOXVjMCtqMDl2Rit1c21GdFJpYVFFY1RLWkI0Q2h6cXZMa3J3ZGc3eWpBWnc9PQ==
  type: tx
- attributes:
  - index: true
    key: YWN0aW9u
    value: L2Nvc213YXNtLndhc20udjEuTXNnSW5zdGFudGlhdGVDb250cmFjdA==
  type: message
- attributes:
  - index: true
    key: bW9kdWxl
    value: d2FzbQ==
  - index: true
    key: c2VuZGVy
    value: c2VpMXE3OWtrend6bXdlbnp6ZGFlNDc0ZXRncXM1Y2pxeGxzaDRjcGFr
  type: message
- attributes:
  - index: true
    key: X2NvbnRyYWN0X2FkZHJlc3M=
    value: c2VpMW5sMmRjaHM2Z3A4dHlkeHQwd3M5ZDI1NDBtMnlydjN1bXp1YTNnYWF6dWdoZDlzNXl3Y3NxOHVnYWg=
  - index: true
    key: Y29kZV9pZA==
    value: MTE0MQ==
  type: instantiate
- attributes:
  - index: true
    key: X2NvbnRyYWN0X2FkZHJlc3M=
    value: c2VpMW5sMmRjaHM2Z3A4dHlkeHQwd3M5ZDI1NDBtMnlydjN1bXp1YTNnYWF6dWdoZDlzNXl3Y3NxOHVnYWg=
  - index: true
    key: bWV0aG9k
    value: aW5zdGFudGlhdGU=
  - index: true
    key: b3duZXI=
    value: c2VpMXE3OWtrend6bXdlbnp6ZGFlNDc0ZXRncXM1Y2pxeGxzaDRjcGFr
  - index: true
    key: Y291bnQ=
    value: MA==
  type: wasm
gas_used: "164615"
gas_wanted: "4000000"
height: "15187953"
info: ""
logs:
- events:
  - attributes:
    - key: _contract_address
      value: sei1nl2dchs6gp8tydxt0ws9d2540m2yrv3umzua3gaazughd9s5ywcsq8ugah
    - key: code_id
      value: "1141"
    type: instantiate
  - attributes:
    - key: action
      value: /cosmwasm.wasm.v1.MsgInstantiateContract
    - key: module
      value: wasm
    - key: sender
      value: sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak
    type: message
  - attributes:
    - key: _contract_address
      value: sei1nl2dchs6gp8tydxt0ws9d2540m2yrv3umzua3gaazughd9s5ywcsq8ugah
    - key: method
      value: instantiate
    - key: owner
      value: sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak
    - key: count
      value: "0"
    type: wasm
  log: ""
  msg_index: 0
raw_log: '[{"events":[{"type":"instantiate","attributes":[{"key":"_contract_address","value":"sei1nl2dchs6gp8tydxt0ws9d2540m2yrv3umzua3gaazughd9s5ywcsq8ugah"},{"key":"code_id","value":"1141"}]},{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgInstantiateContract"},{"key":"module","value":"wasm"},{"key":"sender","value":"sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak"}]},{"type":"wasm","attributes":[{"key":"_contract_address","value":"sei1nl2dchs6gp8tydxt0ws9d2540m2yrv3umzua3gaazughd9s5ywcsq8ugah"},{"key":"method","value":"instantiate"},{"key":"owner","value":"sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak"},{"key":"count","value":"0"}]}]}]'
timestamp: ""
tx: null
txhash: 66B2D61EC12C20CFECF6111AFAA95A3C6893F5A4DB99A9896958A2BFB1341165
```

