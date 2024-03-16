use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Attributes {
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Dex {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Attributes,
}
