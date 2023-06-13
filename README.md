# Web3-Dex-Sei

#### Deploy boilerplate smart contract
To deploy a contract, you will first need to compile the contract. Confirm that the prerequisites listed in the CosmWasm docs are met, and then run the following

'''cargo wasm'''
Before uploading, use the rust-optimizer to minimize the size of the binary that will be uploaded
'''
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11
'''