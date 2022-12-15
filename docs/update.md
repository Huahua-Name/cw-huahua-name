#### Upload new contract

`chihuahuad tx wasm store artifacts/cw_huahua_name.wasm --from wallet --node https://chihuahua-testnet-rpc.polkachu.com:443 --chain-id chitestnet-5 --gas-prices 0.25uhuahua --gas auto --gas-adjustment 1.3 -y --output json -b block`

#### Migrate code id
`MIGRATE='{}'`  
`chihuahuad tx wasm migrate chihuahua14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skazn7y 4 "$MIGRATE" --from wallet --node https://chihuahua-testnet-rpc.polkachu.com:443 --chain-id chitestnet-5 --gas-prices 0.25uhuahua --gas auto --gas-adjustment 1.3 --output json -b block`
