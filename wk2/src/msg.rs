use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub tokens: u128,
}

#[cw_serde]
pub enum ExecuteMsg {
    // StoreTokens {}, 
    ForwardTokens { receiver: Addr },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    // #[returns(GetCountResponse)]
    // GetCount {},
}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct GetCountResponse {
//     pub count: i32,
// }
