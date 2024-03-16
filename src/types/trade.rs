use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trade {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Attributes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub block_number: i64,
    pub tx_hash: String,
    pub tx_from_address: String,
    pub from_token_amount: String,
    pub to_token_amount: String,
    pub price_from_in_currency_token: String,
    pub price_to_in_currency_token: String,
    pub price_from_in_usd: String,
    pub price_to_in_usd: String,
    pub block_timestamp: String,
    pub kind: String,
    pub volume_in_usd: String,
    pub from_token_address: String,
    pub to_token_address: String,
}
