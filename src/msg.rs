use crate::state::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub purchase_price: Option<Coin>,
    pub transfer_price: Option<Coin>,
    pub edit_price: Option<Coin>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Register { name: String, bio: String, website: String },
    Transfer { name: String, to: String },
    Refund {},
    Edit { name: String, bio: String, website: String },
    Editconf { purchase_price: Option<Coin>, transfer_price: Option<Coin>, edit_price: Option<Coin> },
}

#[cw_serde]
pub struct MigrateMsg {
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // ResolveAddress returns the current address that the name resolves to
    #[returns(ResolveRecordResponse)]
    ResolveRecord { name: String },
    #[returns(ConfigResponse)]
    Config {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct ResolveRecordResponse {
    pub address: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub purchase_price: Option<Coin>,
    pub transfer_price: Option<Coin>,
    pub edit_price: Option<Coin>,
}

impl From<Config> for ConfigResponse {
    fn from(config: Config) -> ConfigResponse {
        ConfigResponse {
            owner: config.owner,
            purchase_price: config.purchase_price,
            transfer_price: config.transfer_price,
            edit_price: config.edit_price,
        }
    }
}
