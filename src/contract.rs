use cosmwasm_std::{
    entry_point, to_binary, Binary, BankMsg, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Addr,
};

use crate::coin_helpers::assert_sent_sufficient_coin;
use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, MigrateMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse};
use crate::state::{Config, NameRecord, CONFIG, NAME_RESOLVER};

// Name Config
const MIN_NAME_LENGTH: u64 = 3;
const MAX_NAME_LENGTH: u64 = 30;
const MAX_BIO_LENGTH: u64 = 200;
const MAX_WEBSITE_LENGTH: u64 = 100;
// Semantic Versioning
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let owner = msg
        .admin
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok())
        .unwrap_or(info.sender);

    let config = Config {
        owner: owner.clone(),
        purchase_price: msg.purchase_price,
        transfer_price: msg.transfer_price,
        edit_price: msg.edit_price,
    };
    CONFIG.save(deps.storage, &config)?;

    // Use CW2 to set the contract version, this is needed for migrations
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { name, bio, website } => execute_register(deps, env, info, name, bio, website),
        ExecuteMsg::Transfer { name, to } => execute_transfer(deps, env, info, name, to),
        ExecuteMsg::Refund {} => execute_refund(deps, env, info),
        ExecuteMsg::Edit { name, bio, website } => execute_edit(deps, env, info, name, bio, website),
        ExecuteMsg::Editconf { purchase_price, transfer_price, edit_price } => execute_edit_conf(deps, env, info, purchase_price, transfer_price, edit_price),

    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let ver = cw2::get_contract_version(deps.storage)?;

    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME.to_string() {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    // set the new version
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // do any desired state migrations...

    Ok(Response::default())
}

pub fn execute_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    bio: String,
    website: String,
) -> Result<Response, ContractError> {
    // we only need to check here - at point of registration
    validate_name(&name)?;
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.purchase_price)?;

    let key = name.as_bytes();
    let bio_length = bio.len() as u64;
    let website_length = website.len() as u64;

    if (bio_length) > MAX_BIO_LENGTH {
        return Err(ContractError::BioTooLong {
            bio_length,
            max_length: MAX_BIO_LENGTH,
        })
    }

    if (website_length) > MAX_WEBSITE_LENGTH {
        return Err(ContractError::WebsiteTooLong {
            website_length,
            max_length: MAX_WEBSITE_LENGTH,
        })
    }

    if (NAME_RESOLVER.may_load(deps.storage, key)?).is_some() {
        // name is already taken
        return Err(ContractError::NameTaken { name });
    }

    let record = NameRecord {
        owner: info.sender,
        bio: bio,
        website: website
    };

    // name is available
    NAME_RESOLVER.save(deps.storage, key, &record)?;

    Ok(Response::default())
}

pub fn execute_transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    to: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.transfer_price)?;

    let new_owner = deps.api.addr_validate(&to)?;
    let key = name.as_bytes();
    NAME_RESOLVER.update(deps.storage, key, |record| {
        if let Some(mut record) = record {
            if info.sender != record.owner {
                return Err(ContractError::Unauthorized {});
            }

            record.owner = new_owner.clone();
            Ok(record)
        } else {
            Err(ContractError::NameNotExists { name: name.clone() })
        }
    })?;
    Ok(Response::default())
}

pub fn execute_edit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    bio: String,
    website: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.edit_price)?;

    let key = name.as_bytes();
    let bio_length = bio.len() as u64;
    let website_length = website.len() as u64;

    NAME_RESOLVER.update(deps.storage, key, |record| {
        if let Some(mut record) = record {
            if info.sender != record.owner {
                return Err(ContractError::Unauthorized {});
            }

            if (bio_length) > MAX_BIO_LENGTH {
                return Err(ContractError::BioTooLong {
                    bio_length,
                    max_length: MAX_BIO_LENGTH,
                })
            }

            if (website_length) > MAX_WEBSITE_LENGTH {
                return Err(ContractError::WebsiteTooLong {
                    website_length,
                    max_length: MAX_WEBSITE_LENGTH,
                })
            }

            record.bio = bio.clone();
            record.website = website.clone();
            Ok(record)
        } else {
            Err(ContractError::NameNotExists { name: name.clone() })
        }
    })?;
    Ok(Response::default())
}

pub fn execute_edit_conf(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    purchase_price: Option<Coin>,
    transfer_price: Option<Coin>,
    edit_price: Option<Coin>,
) -> Result<Response, ContractError> {
    let get_config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, get_config.transfer_price)?;

    if get_config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // CONFIG.update(deps.storage, FnOnce::<&Config,>);
    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.purchase_price = purchase_price.clone();
        config.transfer_price = transfer_price.clone();
        config.edit_price = edit_price.clone();
        Ok(config)
    })?;

    Ok(Response::default())
}

fn execute_refund(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let balance = deps.querier.query_all_balances(&env.contract.address)?;
    let config = CONFIG.load(deps.storage)?;

    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(send_tokens(balance, "refund", config.owner))
}

fn send_tokens(amount: Vec<Coin>, action: &str, address: Addr) -> Response {
    Response::new()
        .add_message(BankMsg::Send {
            to_address: address.to_string(),
            amount,
        })
        .add_attribute("action", action)
        .add_attribute("to", address.to_string())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ResolveRecord { name } => query_resolver(deps, env, name),
        QueryMsg::Config {} => to_binary::<ConfigResponse>(&CONFIG.load(deps.storage)?.into()),
    }
}

fn query_resolver(deps: Deps, _env: Env, name: String) -> StdResult<Binary> {
    let key = name.as_bytes();

    let address = match NAME_RESOLVER.may_load(deps.storage, key)? {
        Some(record) => Some(String::from(&record.owner)),
        None => None,
    };
    let bio = match NAME_RESOLVER.may_load(deps.storage, key)? {
        Some(record) => Some(String::from(&record.bio)),
        None => None,
    };
    let website = match NAME_RESOLVER.may_load(deps.storage, key)? {
        Some(record) => Some(String::from(&record.website)),
        None => None,
    };

    let resp = ResolveRecordResponse { address, bio, website };

    to_binary(&resp)
}

// let's not import a regexp library and just do these checks by hand
fn invalid_char(c: char) -> bool {
    let is_valid =
        c.is_ascii_digit() || c.is_ascii_lowercase() || (c == '-' /*|| c == '.' || c == '_'*/);
    !is_valid
}

/// validate_name returns an error if the name is invalid
fn validate_name(name: &str) -> Result<(), ContractError> {
    let length = name.len() as u64;
    if (name.len() as u64) < MIN_NAME_LENGTH {
        Err(ContractError::NameTooShort {
            length,
            min_length: MIN_NAME_LENGTH,
        })
    } else if (name.len() as u64) > MAX_NAME_LENGTH {
        Err(ContractError::NameTooLong {
            length,
            max_length: MAX_NAME_LENGTH,
        })
    } else {
        match name.find(invalid_char) {
            None => Ok(()),
            Some(bytepos_invalid_char_start) => {
                let c = name[bytepos_invalid_char_start..].chars().next().unwrap();
                Err(ContractError::InvalidCharacter { c })
            }
        }
    }
}
