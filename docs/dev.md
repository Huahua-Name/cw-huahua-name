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

#### Schema

    cargo schema

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

