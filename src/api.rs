use log::error;
use serde_json::Value;

use crate::limits::{MAX_ADDRESSES, MAX_PAGE};
use crate::validation::{check_addresses, check_page};

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

    pub async fn get(
        &self,
        path: String,
        params: Vec<(String, String)>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        let res = self
            .client
            .get(&url)
            .query(&params)
            .header("Accept", &self.accept_header)
            .send()
            .await?;

        match res.error_for_status_ref() {
            Ok(_) => Ok(res),
            Err(err) => {
                error!("Error: {}", err);
                Err(err)
            }
        }
    }

    pub async fn networks(&self, page: i32) -> Result<Value, reqwest::Error> {
        check_page(&page, MAX_PAGE);
        let path = "/networks".to_string();
        let params = vec![("page".to_string(), page.to_string())];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_dexes(&self, network: &str, page: i32) -> Result<Value, reqwest::Error> {
        check_page(&page, MAX_PAGE);
        let path = format!("/networks/{}/dexes", network);
        let params = vec![("page".to_string(), page.to_string())];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn trending_pools(
        &self,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        check_page(&page, MAX_PAGE);
        let path = "/networks/trending_pools".to_string();
        let include_str = include.join(",");
        let params = vec![
            ("page".to_string(), page.to_string()),
            ("include".to_string(), include_str),
        ];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_trending_pools(
        &self,
        network: &str,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        check_page(&page, MAX_PAGE);
        let path = format!("/networks/{}/trending_pools", network);
        let include_str = include.join(",");
        let params = vec![
            ("page".to_string(), page.to_string()),
            ("include".to_string(), include_str),
        ];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_pool_address(
        &self,
        network: &str,
        address: &str,
        include: Vec<&str>,
    ) -> Result<Value, reqwest::Error> {
        let path = format!("/networks/{}/pools/{}", network, address);
        let include_str = include.join(",");
        let params = vec![("include".to_string(), include_str)];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_pools_multi_address(
        &self,
        network: &str,
        addresses: Vec<&str>,
        include: Vec<&str>,
    ) -> Result<Value, reqwest::Error> {
        check_addresses(&addresses, MAX_ADDRESSES);
        let path = format!("/networks/{}/pools/multi/{}", network, addresses.join(","));
        let include_str = include.join(",");
        let params = vec![("include".to_string(), include_str)];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_pools(
        &self,
        network: &str,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        check_page(&page, MAX_PAGE);
        let path = format!("/networks/{}/pools", network);
        let include_str = include.join(",");
        let params = vec![
            ("include".to_string(), include_str),
            ("page".to_string(), page.to_string()),
        ];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }
    pub async fn network_dex_pools(
        &self,
        network: &str,
        dex: &str,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        check_page(&page, MAX_PAGE);
        let path = format!("/networks/{}/dexes/{}/pools", network, dex);
        let include_str = include.join(",");
        let params = vec![
            ("include".to_string(), include_str),
            ("page".to_string(), page.to_string()),
        ];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_new_pools(
        &self,
        network: &str,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        check_page(&page, MAX_PAGE);
        let path = format!("/networks/{}/new_pools", network);
        let include_str = include.join(",");
        let params = vec![
            ("include".to_string(), include_str),
            ("page".to_string(), page.to_string()),
        ];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn new_pools(&self, include: Vec<&str>, page: i32) -> Result<Value, reqwest::Error> {
        check_page(&page, MAX_PAGE);
        let path = format!("/networks/new_pools");
        let include_str = include.join(",");
        let params = vec![
            ("include".to_string(), include_str),
            ("page".to_string(), page.to_string()),
        ];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn search_network_pool(
        &self,
        query: &str,
        network: &str,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        let path = "/search/pools".to_string();
        let include_str = include.join(",");
        let params = vec![
            ("query".to_string(), query.to_string()),
            ("network".to_string(), network.to_string()),
            ("include".to_string(), include_str),
            ("page".to_string(), page.to_string()),
        ];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_addresses_token_price(
        &self,
        network: &str,
        addresses: Vec<&str>,
    ) -> Result<Value, reqwest::Error> {
        let path = format!(
            "/simple/networks/{}/token_price/{}",
            network,
            addresses.join(",")
        );
        let params = vec![];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_token_pools(
        &self,
        network: &str,
        token_address: &str,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        let path = format!("/networks/{}/token/{}/pools", network, token_address);
        let include_str = include.join(",");
        let params = vec![
            ("include".to_string(), include_str),
            ("page".to_string(), page.to_string()),
        ];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_token(
        &self,
        network: &str,
        address: &str,
        include: Vec<&str>,
    ) -> Result<Value, reqwest::Error> {
        let path = format!("/networks/{}/tokens/{}", network, address);
        let include_str = include.join(",");
        let params = vec![("include".to_string(), include_str)];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_token_multi_address(
        &self,
        network: &str,
        addresses: Vec<&str>,
        include: Vec<&str>,
    ) -> Result<Value, reqwest::Error> {
        check_addresses(&addresses, MAX_ADDRESSES);
        let path = format!("/networks/{}/tokens/multi/{}", network, addresses.join(","));
        let include_str = include.join(",");
        let params = vec![("include".to_string(), include_str)];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_tokens_address_info(
        &self,
        network: &str,
        address: &str,
    ) -> Result<Value, reqwest::Error> {
        let path = format!("/networks/{}/tokens/{}/info", network, address);
        let params = vec![];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn token_info_recently_updated(
        &self,
        include: Vec<&str>,
    ) -> Result<Value, reqwest::Error> {
        let path = "/tokens/info_recently_updated".to_string();
        let include_str = include.join(",");
        let params = vec![("include".to_string(), include_str)];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }

    pub async fn network_pool_trades(
        &self,
        network: &str,
        pool_address: &str,
        trade_volume_in_usd_greater_than: f64,
    ) -> Result<Value, reqwest::Error> {
        let path = format!("/networks/{}/pools/{}/trades", network, pool_address);
        let params = vec![(
            "trade_volume_in_usd_greater_than".to_string(),
            trade_volume_in_usd_greater_than.to_string(),
        )];
        let res = self.get(path, params).await?;
        res.json::<Value>().await
    }
}
