use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Links {
    pub first: String,
    pub last: String,
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Base {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub coingecko_coin_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub coingecko_coin_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub base: Base,
    pub quote: Quote,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Included {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Attributes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub address: Option<String>,
    pub name: String,
    pub symbol: Option<String>,
    pub image_url: Option<String>,
    pub coingecko_coin_id: Option<String>,
    pub coingecko_asset_platform_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeckoTerminalResponse<T> {
    pub data: T,
    pub links: Option<Links>,
    pub meta: Option<Meta>,
    pub included: Option<Vec<Included>>,
}
