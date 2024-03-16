use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pool {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Attributes,
    pub relationships: Relationships,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub base_token_price_usd: String,
    pub base_token_price_native_currency: String,
    pub quote_token_price_usd: String,
    pub quote_token_price_native_currency: String,
    pub base_token_price_quote_token: String,
    pub quote_token_price_base_token: String,
    pub address: String,
    pub name: String,
    pub pool_created_at: String,
    pub fdv_usd: Option<String>,
    pub market_cap_usd: Value,
    pub price_change_percentage: PriceChangePercentage,
    pub transactions: Transactions,
    pub volume_usd: VolumeUsd,
    pub reserve_in_usd: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceChangePercentage {
    pub m5: String,
    pub h1: String,
    pub h6: String,
    pub h24: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transactions {
    pub m5: M5,
    pub m15: M15,
    pub m30: M30,
    pub h1: H1,
    pub h24: H24,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct M5 {
    pub buys: i64,
    pub sells: i64,
    pub buyers: Option<i64>,
    pub sellers: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct M15 {
    pub buys: i64,
    pub sells: i64,
    pub buyers: Option<i64>,
    pub sellers: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct M30 {
    pub buys: i64,
    pub sells: i64,
    pub buyers: Option<i64>,
    pub sellers: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct H1 {
    pub buys: i64,
    pub sells: i64,
    pub buyers: Option<i64>,
    pub sellers: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct H24 {
    pub buys: i64,
    pub sells: i64,
    pub buyers: Option<i64>,
    pub sellers: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeUsd {
    pub m5: String,
    pub h1: String,
    pub h6: String,
    pub h24: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationships {
    pub base_token: BaseToken,
    pub quote_token: QuoteToken,
    pub network: Option<Network>,
    pub dex: Dex,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseToken {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteToken {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dex {
    pub data: Data,
}
