use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Attributes,
    pub relationships: Option<Relationships>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub image_url: String,
    pub coingecko_coin_id: Option<String>,
    pub websites: Vec<String>,
    pub description: Option<String>,
    pub gt_score: Option<f64>,
    pub discord_url: Option<String>,
    pub telegram_handle: Option<String>,
    pub twitter_handle: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationships {
    pub network: Network,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
