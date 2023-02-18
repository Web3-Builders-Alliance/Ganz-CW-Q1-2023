use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// use cosmwasm_std::Addr;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Owner {
    pub owner: Addr,
}

pub struct Token {
    pub name: String,
    pub tokens: Uint128,
}

pub const OWNER: Item<Owner> = Item::new("owner");
pub const CURRENT_BALANCE: Item<Token> = Item::new("current_balance");
pub const TOTAL_RECEIVED: Item<Token> = Item::new("total_received");
pub const TOTAL_FORWARDED: Item<Token> = Item::new("total_forwarded");


// Decided against using Map coz couldnt figure out how do do what I wanted,
// and just use 3 Items instead
// use cw_storage_plus::{Item, Map};
// pub const BALANCES:  Map<&str, Tokens> = Map::new("balances");
