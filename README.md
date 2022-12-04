 
<img src="https://user-images.githubusercontent.com/1071490/205506995-0a4638dc-cd32-4b9a-a6e7-66bb67f55551.jpg" width="100" height="100" />   

# Huahua-name

We have used the nameservice code that we have improved.  
You can find the informations here:  
- [tutorial](https://docs.cosmwasm.com/tutorials/name-service/intro)  
- [source](https://github.com/deus-labs/cw-contracts/tree/main/contracts/nameservice)  

The goal of the application you are building is to let users buy names and to set a value these names resolve to.
The owner of a given name will be the current highest bidder. In this section, you will learn how these simple
 requirements translate to application design.

## Development

    git clone https://github.com/Huahua-Name/cw-huahua-name.git
	cd cw-huahua-name

#### Compilation

Cargo

    RUSTFLAGS='-C link-arg=-s' cargo wasm

Docker (Optimized Compilation)

    docker run --rm -v "$(pwd)":/code \  
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \  
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \  
    cosmwasm/rust-optimizer:0.12.6

#### Upload contract

    chihuahuad tx wasm store artifacts/cw_huahua_name.wasm --from wallet --node https://chihuahua-testnet-rpc.polkachu.com:443 --chain-id chitestnet-5 --gas-prices 0.25uhuahua --gas auto --gas-adjustment 1.3 -y --output json -b block 
    
#### Prepare the instantiation message

    INIT='{"admin": "chihuahua1s4e80p6ty84agy8d8t6kywwc9u734q275cjm03", "purchase_price":{"amount":"100","denom":"uhuahua"},"transfer_price":{"amount":"999","denom":"uhuahua"},"edit_price":{"denom":"uhuahua","amount":"420"}}'

#### Instantiate the contract

    chihuahuad tx wasm instantiate 1 "$INIT" --from wallet --label "HuaName service" --from wallet --node https://chihuahua-testnet-rpc.polkachu.com:443 --chain-id chitestnet-5 --gas-prices 0.25uhuahua --gas auto --gas-adjustment 1.3 -y --output json -b block --admin chihuahua1s4e80p6ty84agy8d8t6kywwc9u734q275cjm03

#### Config query

    CONFIG_QUERY='{"config": {}}'
    ./chihuahuad query wasm contract-state smart {CONTRACT} "$CONFIG_QUERY" --node https://chihuahua-testnet-rpc.polkachu.com:443 --output json

#### Register new name

    REGISTER='{"register":{"name":"huahua-name", "bio":"420", "website":"https://huahua-name.wtf"}}'
    ./chihuahuad tx wasm execute {CONTRACT} "$REGISTER" --amount 100uhuahua --from wallet --node https://chihuahua-testnet-rpc.polkachu.com:443 --chain-id chitestnet-5 --gas-prices 0.25uhuahua --gas auto --gas-adjustment 1.3 --output json -b block
