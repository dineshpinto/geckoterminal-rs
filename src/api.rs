use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Params {
    page: u8,
    include: Vec<String>,
}

pub struct GeckoTerminalAPI {
    client: reqwest::Client,
    base_url: String,
    accept_header: String,
}

impl GeckoTerminalAPI {
    pub fn new() -> GeckoTerminalAPI {
        GeckoTerminalAPI {
            client: reqwest::Client::new(),
            base_url: "https://api.geckoterminal.com/api/v2".to_string(),
            accept_header: "application/json".to_string(),
        }
    }

    pub async fn get(&self, path: String, params: Vec<(&str, &str)>) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        let res = self.client.get(&url)
            .query(&params)
            .header("Accept", &self.accept_header)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn networks(&self, page: &str) -> Result<Value, reqwest::Error> {
        let path = "/networks".to_string();
        let params = vec![("page", page)];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_dexes(&self, network: &str, page: &str) -> Result<Value, reqwest::Error> {
        let path = format!("/networks/{}/dexes", network);
        let params = vec![("page", page)];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn trending_pools(&self, include: Vec<&str>, page: &str) -> Result<Value, reqwest::Error> {
        let path = "/networks/trending_pools".to_string();
        let include_str = include.join(",");
        let params = vec![("page", page), ("include", &include_str)];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }
}