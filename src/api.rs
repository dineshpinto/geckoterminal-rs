use log::error;
use serde_json::Value;

use crate::validation::{
    check_addresses,
    check_include,
    check_page
};

pub struct GeckoTerminalAPI {
    client: reqwest::Client,
    base_url: String,
    accept_header: String,
}

impl GeckoTerminalAPI {

    /// Create a new GeckoTerminalAPI client.
    ///
    /// # Examples
    ///
    /// ```
    /// use geckoterminal_rs::api::GeckoTerminalAPI;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///    let client = GeckoTerminalAPI::new();
    ///    Ok(())
    /// }
    /// ```
    pub fn new() -> GeckoTerminalAPI {
        GeckoTerminalAPI {
            client: reqwest::Client::new(),
            base_url: "https://api.geckoterminal.com/api/v2".to_string(),
            accept_header: "application/json".to_string(),
        }
    }

    /// Make a GET request to the GeckoTerminalAPI.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The path to make the GET request to.
    /// * `params` - The query parameters to include in the GET request.
    pub async fn get(
        &self,
        path: String,
        params: Vec<(String, String)>,
    ) -> Result<Value, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .client
            .get(&url)
            .query(&params)
            .header("Accept", &self.accept_header)
            .send()
            .await?;

        match resp.error_for_status_ref() {
            Ok(_) => {
                Ok(resp.json().await?)
            }
            Err(err) => {
                error!("Error: {}", err);
                Err(err)
            }
        }
    }

    /// Get all supported networks along with their network ID.
    ///
    /// # Arguments
    ///
    /// * `page` - The page number of the results to return. Default is 1.
    pub async fn networks(&self, page: i32) -> Result<Value, reqwest::Error> {
        check_page(&page);
        let path = "/networks".to_string();
        let params = vec![("page".to_string(), page.to_string())];
        self.get(path, params).await
    }

    /// Get all supported DEXes along with their DEX ID.
    ///
    /// # Arguments
    ///
    /// * `network` - The network ID of the network to get DEXes for.
    /// * `page` - The page number of the results to return. Default is 1.
    pub async fn network_dexes(&self, network: &str, page: i32) -> Result<Value, reqwest::Error> {
        check_page(&page);
        let path = format!("/networks/{}/dexes", network);
        let params = vec![("page".to_string(), page.to_string())];
        self.get(path, params).await
    }

    /// Get all trending pools on all networks.
    ///
    /// # Arguments
    ///
    /// * `include` - List of related resources to include in response. Available
    /// resources are: base_token, quote_token, dex, network (default all).
    /// * `page` - The page number of the results to return. Default is 1.
    pub async fn trending_pools(
        &self,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        check_page(&page);
        check_include(&include, "pool");
        let path = "/networks/trending_pools".to_string();
        let include_str = include.join(",");
        let params = vec![
            ("page".to_string(), page.to_string()),
            ("include".to_string(), include_str),
        ];
        self.get(path, params).await
    }

    /// Get all trending pools on a specific network.
    ///
    /// # Arguments
    ///
    /// * `network` - The network ID of the network to get trending pools for.
    /// * `include` - List of related resources to include in response. Available
    /// resources are: base_token, quote_token, dex (default all).
    /// * `page` - The page number of the results to return. Default is 1.
    pub async fn network_trending_pools(
        &self,
        network: &str,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        check_page(&page);
        check_include(&include, "network_pool");
        let path = format!("/networks/{}/trending_pools", network);
        let include_str = include.join(",");
        let params = vec![
            ("page".to_string(), page.to_string()),
            ("include".to_string(), include_str),
        ];
        self.get(path, params).await
    }

    /// Get a specific pool on a specific network.
    ///
    /// # Arguments
    ///
    /// * `network` - The network ID of the network to get the pool for.
    /// * `address` - The address of the pool to get.
    /// * `include` - List of related resources to include in response. Available
    /// resources are: base_token, quote_token, dex (default all).
    pub async fn network_pool_address(
        &self,
        network: &str,
        address: &str,
        include: Vec<&str>,
    ) -> Result<Value, reqwest::Error> {
        check_include(&include, "network_pool");
        let path = format!("/networks/{}/pools/{}", network, address);
        let include_str = include.join(",");
        let params = vec![("include".to_string(), include_str)];
        self.get(path, params).await
    }

    /// Get multiple pools on a specific network.
    ///
    /// # Arguments
    ///
    /// * `network` - The network ID of the network to get the pools for.
    /// * `addresses` - The addresses of the pools to get.
    /// * `include` - List of related resources to include in response. Available
    /// resources are: base_token, quote_token, dex (default all).
    pub async fn network_pools_multi_address(
        &self,
        network: &str,
        addresses: Vec<&str>,
        include: Vec<&str>,
    ) -> Result<Value, reqwest::Error> {
        check_addresses(&addresses);
        check_include(&include, "network_pool");
        let path = format!("/networks/{}/pools/multi/{}", network, addresses.join(","));
        let include_str = include.join(",");
        let params = vec![("include".to_string(), include_str)];
        self.get(path, params).await
    }

    /// Get all pools on a specific network.
    ///
    /// # Arguments
    ///
    /// * `network` - The network ID of the network to get the pools for.
    /// * `include` - List of related resources to include in response. Available
    /// resources are: base_token, quote_token, dex (default all).
    /// * `page` - The page number of the results to return. Default is 1.
    pub async fn network_pools(
        &self,
        network: &str,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        check_page(&page);
        check_include(&include, "network_pool");
        let path = format!("/networks/{}/pools", network);
        let include_str = include.join(",");
        let params = vec![
            ("include".to_string(), include_str),
            ("page".to_string(), page.to_string()),
        ];
        self.get(path, params).await
    }

    /// Get top pools on a network's DeX.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the pools for.
    /// * `dex` - The DeX ID of the DeX to get the pools for.
    /// * `include` - List of related resources to include in response. Available
    /// resources are: base_token, quote_token (default all).
    /// * `page` - The page number of the results to return. Default is 1.
    pub async fn network_dex_pools(
        &self,
        network: &str,
        dex: &str,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        check_include(&include, "network_pool");
        check_page(&page);
        let path = format!("/networks/{}/dexes/{}/pools", network, dex);
        let include_str = include.join(",");
        let params = vec![
            ("include".to_string(), include_str),
            ("page".to_string(), page.to_string()),
        ];
        self.get(path, params).await
    }

    /// Get new pools on a network.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the pools for.
    /// * `include` - List of related resources to include in response. Available
    /// resources are: base_token, quote_token, dex (default all).
    /// * `page` - The page number of the results to return. Default is 1.
    pub async fn network_new_pools(
        &self,
        network: &str,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        check_include(&include, "network_pool");
        check_page(&page);
        let path = format!("/networks/{}/new_pools", network);
        let include_str = include.join(",");
        let params = vec![
            ("include".to_string(), include_str),
            ("page".to_string(), page.to_string()),
        ];
        self.get(path, params).await
    }

    /// Get new pools on all networks.
    ///
    /// # Arguments
    /// * `include` - List of related resources to include in response. Available
    /// resources are: base_token, quote_token, dex, network (default all).
    /// * `page` - The page number of the results to return. Default is 1.
    pub async fn new_pools(&self, include: Vec<&str>, page: i32) -> Result<Value, reqwest::Error> {
        check_include(&include, "pool");
        check_page(&page);
        let path = "/networks/new_pools".to_string();
        let include_str = include.join(",");
        let params = vec![
            ("include".to_string(), include_str),
            ("page".to_string(), page.to_string()),
        ];
        self.get(path, params).await
    }

    /// Search for a pool on a networks.
    ///
    /// # Arguments
    /// * `query` - The query string to search for, can be pool address, token address, or token symbol.
    /// * `network` - The network ID of the network to search on.
    /// * `include` - List of related resources to include in response. Available
    /// resources are: base_token, quote_token, dex, network (default all).
    /// * `page` - The page number of the results to return. Default is 1.
    pub async fn search_network_pool(
        &self,
        query: &str,
        network: &str,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        check_include(&include, "network_pool");
        check_page(&page);
        let path = "/search/pools".to_string();
        let include_str = include.join(",");
        let params = vec![
            ("query".to_string(), query.to_string()),
            ("network".to_string(), network.to_string()),
            ("include".to_string(), include_str),
            ("page".to_string(), page.to_string()),
        ];
        self.get(path, params).await
    }

    /// Get current USD prices of multiple tokens on a network.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the token prices for.
    /// * `addresses` - The addresses of the tokens to get the prices for.
    pub async fn network_addresses_token_price(
        &self,
        network: &str,
        addresses: Vec<&str>,
    ) -> Result<Value, reqwest::Error> {
        check_addresses(&addresses);
        let path = format!(
            "/simple/networks/{}/token_price/{}",
            network,
            addresses.join(",")
        );
        let params = vec![];
        self.get(path, params).await
        
    }

    /// Get top pools for a token on a network.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the token prices for.
    /// * `token_address` - The address of the token to get the pools for.
    /// * `include` - List of related resources to include in response. Available
    /// resources are: base_token, quote_token, dex (default all).
    pub async fn network_token_pools(
        &self,
        network: &str,
        token_address: &str,
        include: Vec<&str>,
        page: i32,
    ) -> Result<Value, reqwest::Error> {
        check_include(&include, "network_pool");
        check_page(&page);
        let path = format!("/networks/{}/token/{}/pools", network, token_address);
        let include_str = include.join(",");
        let params = vec![
            ("include".to_string(), include_str),
            ("page".to_string(), page.to_string()),
        ];
        self.get(path, params).await
    }

    /// Get specific token on a network.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the token for.
    /// * `address` - The address of the token to get.
    /// * `include` - List of related resources to include in response. Available
    /// resources are: top_pools (default all).
    pub async fn network_token(
        &self,
        network: &str,
        address: &str,
        include: Vec<&str>,
    ) -> Result<Value, reqwest::Error> {
        check_include(&include, "token");
        let path = format!("/networks/{}/tokens/{}", network, address);
        let include_str = include.join(",");
        let params = vec![("include".to_string(), include_str)];
        self.get(path, params).await
    }

    /// Get multiple tokens on a network.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the tokens for.
    /// * `addresses` - The addresses of the tokens to get.
    /// * `include` - List of related resources to include in response. Available
    /// resources are: top_pools (default all).
    pub async fn network_token_multi_address(
        &self,
        network: &str,
        addresses: Vec<&str>,
        include: Vec<&str>,
    ) -> Result<Value, reqwest::Error> {
        check_addresses(&addresses);
        check_include(&include, "token");
        let path = format!("/networks/{}/tokens/multi/{}", network, addresses.join(","));
        let include_str = include.join(",");
        let params = vec![("include".to_string(), include_str)];
        self.get(path, params).await
    }

    /// Get token address info on a network.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the token address info for.
    /// * `address` - The address of the token to get the address info for.
    pub async fn network_tokens_address_info(
        &self,
        network: &str,
        address: &str,
    ) -> Result<Value, reqwest::Error> {
        let path = format!("/networks/{}/tokens/{}/info", network, address);
        let params = vec![];
        self.get(path, params).await
    }

    /// Get most recently updated 100 tokens info from all networks.
    ///
    /// # Arguments
    /// * `include` - List of related resources to include in response. Available
    /// resources are: network (default all).
    pub async fn token_info_recently_updated(
        &self,
        include: Vec<&str>,
    ) -> Result<Value, reqwest::Error> {
        check_include(&include, "token_info");
        let path = "/tokens/info_recently_updated".to_string();
        let include_str = include.join(",");
        let params = vec![("include".to_string(), include_str)];
        self.get(path, params).await
    }

    /// Get trades of a pool
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the trades for.
    /// * `pool_address` - The address of the pool to get the trades for.
    /// * `trade_volume_in_usd_greater_than` - The minimum trade volume in USD to filter by.
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
        self.get(path, params).await
    }
}
