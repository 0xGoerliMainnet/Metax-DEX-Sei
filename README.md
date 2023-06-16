# Web3-Dex-Sei

## Compile

Confirm that the prerequisites listed in the CosmWasm docs are met, and then run the following

```
cargo wasm
```

Before uploading, use the rust-optimizer to minimize the size of the binary that will be uploaded


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

seid tx wasm store $CONTRACT_WASM_BINARY -y --from=$ACCOUNT_NAME --node $ENDPOINT --chain-id=$CHAINID --gas=1203276 --fees=120337usei --broadcast-mode=block

···

Once your proposal is stored to the testnet, you can instantiate your contract:

```
export CONTRACT_ID=1182
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

raw_log: '[{"events":[{"type":"instantiate","attributes":[{"key":"_contract_address","value":"sei1nl2dchs6gp8tydxt0ws9d2540m2yrv3umzua3gaazughd9s5ywcsq8ugah"},{"key":"code_id","value":"1141"}]},{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgInstantiateContract"},{"key":"module","value":"wasm"},{"key":"sender","value":"sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak"}]},{"type":"wasm","attributes":[{"key":"_contract_address","value":"sei1nl2dchs6gp8tydxt0ws9d2540m2yrv3umzua3gaazughd9s5ywcsq8ugah"},{"key":"method","value":"instantiate"},{"key":"owner","value":"sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak"},{"key":"count","value":"0"}]}]}]'
timestamp: ""
tx: null
txhash: 66B2D61EC12C20CFECF6111AFAA95A3C6893F5A4DB99A9896958A2BFB1341165
```

#### Execute

**sparrow swap exec**

```
export CONTRACT=sei1dgs47p8fe384pepp4q09fqwxu0xpr99j69d7avhqkfs5vsyzvl2sajz57m
export ARGS='{"swap":{"offer_asset":{"info":{"native_token":{"denom":"usei"}},"amount":"5000"},"belief_price":"773292751","max_spread":"0.05"}}'
export ACCOUNT_NAME=shaneson

seid tx wasm execute $CONTRACT $ARGS --from $ACCOUNT_NAME --broadcast-mode=block --chain-id atlantic-2 --gas=351964 --fees=35198usei --node $ENDPOINT --amount 5000usei -y

 ```


 **wasm-dexrouter swap exec -- sparrowswap**

```
export CONTRACT=sei1dyqkrx2kvg7pgj5674gq3flj55hql3fy8elvlyssuwrgw5sp5f3qsa3xh2
export ARGS='{"sparrow_swap":{"pool_address":"sei1dgs47p8fe384pepp4q09fqwxu0xpr99j69d7avhqkfs5vsyzvl2sajz57m", "offer_asset":{"info":{"native_token":{"denom":"usei"}},"amount":"5000"},"belief_price":"773292751","max_spread":"0.05", "to": "sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak" }}'
export ACCOUNT_NAME=shaneson

seid tx wasm execute $CONTRACT $ARGS --from $ACCOUNT_NAME --broadcast-mode=block --chain-id atlantic-2 --gas=351964 --fees=35198usei --node $ENDPOINT --amount 5000usei -y

 ```


 **wasm-dexrouter swap exec -- astropswap**

```
export CONTRACT=sei19x8whl2khg0qqvq67yr00h6t36tau94s77e25ea8vaa0wpw6jqfs5d9vmy
export ARGS='{"astroport_swap": {"pool_address": "sei1uvstll9af7947guj2zs7ywr328twrhhrc6jwenrq68nk8e829z6qttjfe6","offer_asset": {"info": {"native_token": {"denom": "usei"}},"amount": "50000"},"max_spread": "0.05","belief_price": "0.0079479400362710", "to": "sei1q79kkzwzmwenzzdae474etgqs5cjqxlsh4cpak"}}'
export ACCOUNT_NAME=shaneson

seid tx wasm execute $CONTRACT $ARGS --from $ACCOUNT_NAME --broadcast-mode=block --chain-id atlantic-2 --gas=5020450 --fees=502900usei --node $ENDPOINT --amount 50000usei -y

 ```

 