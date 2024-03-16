use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Attributes {
    pub name: String,
    pub coingecko_asset_platform_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Network {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Attributes,
}
