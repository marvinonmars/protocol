use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub ma_token_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    InitAsset {
        /// Symbol used in Terra (e.g: luna, usd)
        symbol: String,
    },
    InitAssetTokenCallback {
        /// Either the Ticker for a terra native asset or address for a cw20 token
        id: String,
    },
    /// Deposit Terra native coins
    DepositNative {
        /// Symbol used in Terra (e.g: luna, usd)
        symbol: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetConfig {},
    QueryReserve { symbol: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub ma_token_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReserveResponse {
    pub ma_token_address: HumanAddr,
}
