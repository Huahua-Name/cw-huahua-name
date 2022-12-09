
<div align="center">
<img src="https://user-images.githubusercontent.com/1071490/206801143-81eef3ef-4bdd-4d8d-8607-ef3a8b7cf39e.jpg" width="600" height="436" />
</div>

# Huahua-name

*   [What is Huahua-name?](#what-is-huahua-name "What is Huahua-name?")
*   [Current contract](#current-contract "Current contract")
*   [Current config](#current-config "Current config")
*   [Check smartcontract](#check-smartcontract "Check smartcontract")
*   [Use contract](#use-contract "Use contract")
*   [Credits](#credits "Credits")

### What is Huahua-name? ###

The purpose of this smartcontract is to assign a name to an address with personalized data.  
This data is editable only by the name administrator.  
It is also possible to transfer the names to another address.

You can perform these actions from chihuahuad or from our interface on [huahua-name.wtf](https://huahua-name.wtf/)  


### Current contract  ###

| Key | Value |
|--|--|
| Network  | Mainnet |
| Chain id | chihuahua-1 |
| Code id  | 157 |
| Contract admin |chihuahua1s4e80p6ty84agy8d8t6kywwc9u734q275cjm03 |
| Contract address |chihuahua1taheysd5agm8jxh3zv8r545nazsg2ye0samdn55g7jexnsz678hqwpzwz9 |
| Instantiate tx |C9B9F9B47190CFB5064ED03B362997BA1CE5452FFEC256BA6F3B169305181688 |

### Current config ###

| Key | Value |
|--|--|
| purchase_price  | 20000 HUAHUA |
| transfer_price | 20000 HUAHUA |
| edit_price | 1000 HUAHUA | 


### Check smartcontract ###

Donwload curent smartcontract:

```
chihuahuad query wasm code 157 download.wasm
openssl sha1 download.wasm
```

Result
```
SHA1(download.wasm)= fbf78f7cab54af4cd2c990512b98140a9c152f2c
```  

Compare the sha1 of the online contract with the github sources

```
openssl sha1 artifacts/cw_huahua_name.wasm
```

Result
```
SHA1(artifacts/cw_huahua_name.wasm)= fbf78f7cab54af4cd2c990512b98140a9c152f2c
```

The contract used on the chihuahua mainnet is exactly the same as the github sources!


### Use contract ###

Soon

### Credits ###

We have used the [nameservice](https://github.com/deus-labs/cw-contracts/tree/main/contracts/nameservice) code that we have improved.
