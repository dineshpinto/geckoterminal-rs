use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Attributes,
    pub relationships: Relationships,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub image_url: String,
    pub coingecko_coin_id: String,
    pub decimals: i64,
    pub total_supply: String,
    pub price_usd: String,
    pub fdv_usd: String,
    pub total_reserve_in_usd: String,
    pub volume_usd: VolumeUsd,
    pub market_cap_usd: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeUsd {
    pub h24: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationships {
    pub top_pools: TopPools,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopPools {
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Included {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Attributes2,
    pub relationships: Relationships2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes2 {
    pub base_token_price_usd: String,
    pub base_token_price_native_currency: String,
    pub quote_token_price_usd: String,
    pub quote_token_price_native_currency: String,
    pub base_token_price_quote_token: String,
    pub quote_token_price_base_token: String,
    pub address: String,
    pub name: String,
    pub pool_created_at: String,
    pub token_price_usd: String,
    pub fdv_usd: String,
    pub market_cap_usd: String,
    pub price_change_percentage: PriceChangePercentage,
    pub transactions: Transactions,
    pub volume_usd: VolumeUsd2,
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
pub struct VolumeUsd2 {
    pub m5: String,
    pub h1: String,
    pub h6: String,
    pub h24: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationships2 {
    pub base_token: BaseToken,
    pub quote_token: QuoteToken,
    pub dex: Dex,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseToken {
    pub data: Data2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data2 {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteToken {
    pub data: Data3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data3 {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dex {
    pub data: Data4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data4 {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
