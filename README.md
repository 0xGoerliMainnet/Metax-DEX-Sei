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
export ENDPOINT=https://sei-testnet-rpc.polkachu.com/
export CHAINID=atlantic-2

seid tx wasm store $CONTRACT_WASM_BINARY -y --from=$ACCOUNT_NAME --node $ENDPOINT --chain-id=$CHAINID --gas=1568479 --fees=150337usei --broadcast-mode=block

```

Once your proposal is stored to the testnet, you can instantiate your contract:

```
export CONTRACT_ID=1285
export LABEL="wasm-dexrouter"
seid tx wasm instantiate $CONTRACT_ID '{"count": 0}' --chain-id $CHAINID --from $ACCOUNT_NAME --gas=4000000 --fees=50000usei --broadcast-mode=block --label $LABEL --admin $ACCOUNT_ADDRESS --node $ENDPOINT
```

#### Execute

**sparrow swap exec**

```
export CONTRACT=sei120n77c9gwhz5hhmuw8n8sprmq485y9d3gggj3skdly38sl77jglsgsdm3j
export ARGS='{"swap":{"offer_asset":{"info":{"native_token":{"denom":"usei"}},"amount":"5000"},"belief_price":"773292751","max_spread":"0.05"}}'
export ACCOUNT_NAME=shaneson

seid tx wasm execute $CONTRACT $ARGS --from $ACCOUNT_NAME --broadcast-mode=block --chain-id atlantic-2 --gas=351964 --fees=35198usei --node $ENDPOINT --amount 5000usei -y

 ```


 **wasm-dexrouter swap exec -- sparrowswap**

```
export CONTRACT=sei19x8whl2khg0qqvq67yr00h6t36tau94s77e25ea8vaa0wpw6jqfs5d9vmy
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

 **wasm-dexrouter unxswap one step**
 

```
export CONTRACT=sei1clacnmdlfclvg2cq76xgz9vcdyyuqhcqnn6q23klt94drcsnm3dsghdllr
export ARGS='{"unxswap": {"steps": [{"sparrow_swap":{"pool_address":"sei1dgs47p8fe384pepp4q09fqwxu0xpr99j69d7avhqkfs5vsyzvl2sajz57m", "offer_asset":{"info":{"native_token":{"denom":"usei"}},"amount":"5000"}, "base_swap_info": {"native_swap": {"offer_denom": "usei", "ask_denom": "factory/sei135mlnw9ndkyglgx7ma95pw22cl64mpnw58pfpd/usdc"}} }}], "minimum_receive": "0" }}'

seid tx wasm execute $CONTRACT $ARGS --from $ACCOUNT_NAME --broadcast-mode=block --chain-id atlantic-2 --gas=493206 --fees=49198usei --node $ENDPOINT --amount 5000usei -y

```

 **wasm-dexrouter unxswap two step**

```
export CONTRACT=sei13aherqkk66eu6mgeykx9j48vu5erclcgk2lt0c5ngnk23rcjr2tsq5wx04
export ARGS='{"unxswap": {"steps": [{"sparrow_swap":{"pool_address":"sei1dgs47p8fe384pepp4q09fqwxu0xpr99j69d7avhqkfs5vsyzvl2sajz57m", "offer_asset_info": {"native_token":  {"denom":"usei"}}}},{"sparrow_swap":{"pool_address":"sei1dgs47p8fe384pepp4q09fqwxu0xpr99j69d7avhqkfs5vsyzvl2sajz57m", "offer_asset_info": {"native_token":{"denom":"factory/sei135mlnw9ndkyglgx7ma95pw22cl64mpnw58pfpd/usdc"}} }}], "minimum_receive": "0", "target_asset_info": {"native_token":{"denom":"usei"}} } }'

seid tx wasm execute $CONTRACT $ARGS --from $ACCOUNT_NAME --broadcast-mode=block --chain-id atlantic-2 --gas=1493206 --fees=149198usei --node $ENDPOINT --amount 5000usei -y

txhash: 64A2643FF2E89C6EAD033C3DE68F314A464AFA18B07CA226667DB44FE933DF6B
```
 