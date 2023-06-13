# Web3-Dex-Sei

#### Compile

Confirm that the prerequisites listed in the CosmWasm docs are met, and then run the following

```
cargo wasm
```
Before uploading, use the rust-optimizer to minimize the size of the binary that will be uploaded
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11
```

#### Deploy

Store a compiled wasm binary to the sei network:

```
seid tx wasm store $CONTRACT_WASM_BINARY -y --from=$ACCOUNT_NAME --chain-id=$CHAIN_ID --node $ENDPOINT --gas=10000000 --fees=100000usei --broadcast-mode=block
```

Once your proposal is stored to the testnet, you can instantiate your contract:

```
seid tx wasm instantiate $CONTRACT_ID '{}' --chain-id sei-chain --from $ACCOUNT_NAME --gas=4000000 --fees=1000000usei --broadcast-mode=block --label $LABEL --no-admin
```

Note that the '{}' part is the parameters you pass to instantiate the contract. In this example, the contract takes no parameter so '{}' suffices. For any real world contracts, their instantiation parameters would likely be non-empty. 
You should get a response like the following:

```
height: "2051"
info: ""
logs:
- events:
  - attributes:
    - key: _contract_address
      value: sei1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqdxfzff
    - key: code_id
      value: "1"
    type: instantiate
  - attributes:
    - key: action
      value: /cosmwasm.wasm.v1.MsgInstantiateContract
    - key: module
      value: wasm
    - key: sender
      value: cosmos1ep9jyk9kydjz0fhadm7rzy6pc9ga7tdt4d26xn
    type: message
  log: ""
  msg_index: 0
raw_log: '[{"events":[{"type":"instantiate","attributes":[{"key":"_contract_address","value":"cosmos1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqdxfzff"},{"key":"code_id","value":"2"}]},{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgInstantiateContract"},{"key":"module","value":"wasm"},{"key":"sender","value":"cosmos1ep9jyk9kydjz0fhadm7rzy6pc9ga7tdt4d26xn"}]}]}]'
timestamp: ""
tx: null
txhash: 3033A3673367169693157DF4D22D973A012C03A4FBE452E994E4DE87C8628D23
```

