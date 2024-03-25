use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::{
    dex::Dex, network::Network, ohlcv::OHLCV, pool::Pool, response::GeckoTerminalResponse,
    simple::TokenPrice, token::Token, token_info::TokenInfo, trade::Trade,
};
use crate::validation::{
    check_addresses, check_aggregate, check_currency, check_ohlcv_limit, check_page,
    check_timeframe, check_token,
};

pub mod limits;
pub mod types;
pub mod validation;

pub struct GeckoTerminalAPI {
    client: reqwest::Client,
    base_url: String,
    accept_header: String,
}

impl Default for GeckoTerminalAPI {
    fn default() -> Self {
        GeckoTerminalAPI {
            client: reqwest::Client::new(),
            base_url: "https://api.geckoterminal.com/api/v2".to_string(),
            accept_header: "application/json".to_string(),
        }
    }
}

impl GeckoTerminalAPI {
    /// Create a new `GeckoTerminalAPI` client.
    ///
    /// # Examples
    ///
    /// ```
    /// use geckoterminal_rs::GeckoTerminalAPI;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///    let gt = GeckoTerminalAPI::new();
    ///    Ok(())
    /// }
    /// ```
    #[must_use]
    pub fn new() -> GeckoTerminalAPI {
        GeckoTerminalAPI::default()
    }

    /// Make a GET request to the `GeckoTerminalAPI`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to make the GET request to.
    /// * `params` - The query parameters to include in the GET request.
    async fn get(&self, path: String, params: Value) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .client
            .get(&url)
            .query(&params)
            .header("Accept", &self.accept_header)
            .send()
            .await;

        match resp {
            Ok(resp) => match resp.error_for_status() {
                Ok(r) => Ok(r),
                Err(err) => {
                    log::error!("Error: {}", err);
                    Err(err)
                }
            },
            Err(err) => {
                log::error!("Error: {}", err);
                Err(err)
            }
        }
    }

    /// This function is used to format the response from the `GeckoTerminalAPI`.
    ///
    /// # Arguments
    ///
    /// * `resp` - The `reqwest::Response` to format.
    async fn format_response<T: serde::de::DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<GeckoTerminalResponse<T>, reqwest::Error> {
        resp.json::<GeckoTerminalResponse<T>>().await
    }

    /// Get all supported networks along with their network ID.
    ///
    /// # Arguments
    ///
    /// * `page` - The page number of the results to return.
    pub async fn networks(
        &self,
        page: Option<i32>,
    ) -> Result<GeckoTerminalResponse<Vec<Network>>, reqwest::Error> {
        let page = page.unwrap_or(1);
        check_page(&page);
        let path = "/networks".to_string();
        let params = json!({ "page": page });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Network>>(resp).await
    }

    /// Get all supported dexes along with their dex ID.
    ///
    /// # Arguments
    ///
    /// * `network` - The network ID of the network to get dexes for.
    /// * `page` - The page number of the results to return.
    pub async fn network_dexes(
        &self,
        network: &str,
        page: Option<i32>,
    ) -> Result<GeckoTerminalResponse<Vec<Dex>>, reqwest::Error> {
        let page = page.unwrap_or(1);
        check_page(&page);
        let path = format!("/networks/{network}/dexes");
        let params = json!({ "page": page });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Dex>>(resp).await
    }

    /// Get all trending pools on all networks.
    ///
    /// # Arguments
    ///
    /// * `page` - The page number of the results to return.
    pub async fn trending_pools(
        &self,
        page: Option<i32>,
    ) -> Result<GeckoTerminalResponse<Vec<Pool>>, reqwest::Error> {
        let page = page.unwrap_or(1);
        check_page(&page);
        let path = "/networks/trending_pools".to_string();
        let include_str = "base_token,quote_token,dex,network";
        let params = json!({ "page": page , "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Pool>>(resp).await
    }

    /// Get all trending pools on a specific network.
    ///
    /// # Arguments
    ///
    /// * `network` - The network ID of the network to get trending pools for.
    /// * `page` - The page number of the results to return.
    pub async fn network_trending_pools(
        &self,
        network: &str,
        page: Option<i32>,
    ) -> Result<GeckoTerminalResponse<Vec<Pool>>, reqwest::Error> {
        let page = page.unwrap_or(1);
        check_page(&page);
        let path = format!("/networks/{network}/trending_pools");
        let include_str = "base_token,quote_token,dex";
        let params = json!({ "page": page , "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Pool>>(resp).await
    }

    /// Get a specific pool on a specific network.
    ///
    /// # Arguments
    ///
    /// * `network` - The network ID of the network to get the pool for.
    /// * `address` - The address of the pool to get.
    pub async fn network_pool_address(
        &self,
        network: &str,
        address: &str,
    ) -> Result<GeckoTerminalResponse<Pool>, reqwest::Error> {
        let path = format!("/networks/{network}/pools/{address}");
        let include_str = "base_token,quote_token,dex";
        let params = json!({ "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Pool>(resp).await
    }

    /// Get multiple pools on a specific network.
    ///
    /// # Arguments
    ///
    /// * `network` - The network ID of the network to get the pools for.
    /// * `addresses` - The addresses of the pools to get.
    pub async fn network_pools_multi_address(
        &self,
        network: &str,
        addresses: Vec<&str>,
    ) -> Result<GeckoTerminalResponse<Vec<Pool>>, reqwest::Error> {
        check_addresses(&addresses);
        let path = format!("/networks/{network}/pools/multi/{}", addresses.join(","));
        let include_str = "base_token,quote_token,dex";
        let params = json!({ "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Pool>>(resp).await
    }

    /// Get all pools on a specific network.
    ///
    /// # Arguments
    ///
    /// * `network` - The network ID of the network to get the pools for.
    /// * `page` - The page number of the results to return.
    pub async fn network_pools(
        &self,
        network: &str,
        page: Option<i32>,
    ) -> Result<GeckoTerminalResponse<Vec<Pool>>, reqwest::Error> {
        let page = page.unwrap_or(1);
        check_page(&page);
        let path = format!("/networks/{network}/pools");
        let include_str = "base_token,quote_token,dex";
        let params = json!({ "page": page , "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Pool>>(resp).await
    }

    /// Get top pools on a network's dex.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the pools for.
    /// * `dex` - The dex ID of the dex to get the pools for.
    /// * `page` - The page number of the results to return.
    pub async fn network_dex_pools(
        &self,
        network: &str,
        dex: &str,
        page: Option<i32>,
    ) -> Result<GeckoTerminalResponse<Vec<Pool>>, reqwest::Error> {
        let page = page.unwrap_or(1);
        check_page(&page);
        let path = format!("/networks/{network}/dexes/{dex}/pools");
        let include_str = "base_token,quote_token,dex";
        let params = json!({ "page": page , "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Pool>>(resp).await
    }

    /// Get new pools on a network.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the pools for.
    /// * `page` - The page number of the results to return.
    pub async fn network_new_pools(
        &self,
        network: &str,
        page: Option<i32>,
    ) -> Result<GeckoTerminalResponse<Vec<Pool>>, reqwest::Error> {
        let page = page.unwrap_or(1);
        check_page(&page);
        let path = format!("/networks/{network}/new_pools");
        let include_str = "base_token,quote_token,dex";
        let params = json!({ "page": page , "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Pool>>(resp).await
    }

    /// Get new pools on all networks.
    ///
    /// # Arguments
    /// * `page` - The page number of the results to return.
    pub async fn new_pools(
        &self,
        page: Option<i32>,
    ) -> Result<GeckoTerminalResponse<Vec<Pool>>, reqwest::Error> {
        let page = page.unwrap_or(1);
        check_page(&page);
        let path = "/networks/new_pools".to_string();
        let include_str = "base_token,quote_token,dex,network";
        let params = json!({ "page": page , "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Pool>>(resp).await
    }

    /// Search for a pool on a networks.
    ///
    /// # Arguments
    /// * `query` - The query string to search for, can be pool address, token address, or token symbol.
    /// * `network` - The network ID of the network to search on.
    /// * `page` - The page number of the results to return.
    pub async fn search_network_pool(
        &self,
        query: &str,
        network: &str,
        page: Option<i32>,
    ) -> Result<GeckoTerminalResponse<Vec<Pool>>, reqwest::Error> {
        let page = page.unwrap_or(1);
        check_page(&page);
        let path = "/search/pools".to_string();
        let include_str = "base_token,quote_token,dex";
        let params =
            json!({ "query": query, "network": network, "page": page , "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Pool>>(resp).await
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
    ) -> Result<GeckoTerminalResponse<TokenPrice>, reqwest::Error> {
        check_addresses(&addresses);
        let path = format!(
            "/simple/networks/{network}/token_price/{}",
            addresses.join(",")
        );
        let params = json!({});
        let resp = self.get(path, params).await?;
        self.format_response::<TokenPrice>(resp).await
    }

    /// Get top pools for a token on a network.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the token prices for.
    /// * `token_address` - The address of the token to get the pools for.
    pub async fn network_token_pools(
        &self,
        network: &str,
        token_address: &str,
        page: Option<i32>,
    ) -> Result<GeckoTerminalResponse<Vec<Pool>>, reqwest::Error> {
        let page = page.unwrap_or(1);
        check_page(&page);
        let path = format!("/networks/{network}/tokens/{token_address}/pools");
        let include_str = "base_token,quote_token,dex";
        let params = json!({ "page": page , "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Pool>>(resp).await
    }

    /// Get specific token on a network.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the token for.
    /// * `address` - The address of the token to get.
    pub async fn network_token(
        &self,
        network: &str,
        address: &str,
    ) -> Result<GeckoTerminalResponse<Token>, reqwest::Error> {
        let path = format!("/networks/{network}/tokens/{address}");
        let include_str = "top_pools";
        let params = json!({ "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Token>(resp).await
    }

    /// Get multiple tokens on a network.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the tokens for.
    /// * `addresses` - The addresses of the tokens to get.
    pub async fn network_token_multi_address(
        &self,
        network: &str,
        addresses: Vec<&str>,
    ) -> Result<GeckoTerminalResponse<Vec<Token>>, reqwest::Error> {
        check_addresses(&addresses);
        let path = format!("/networks/{network}/tokens/multi/{}", addresses.join(","));
        let include_str = "top_pools";
        let params = json!({ "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Token>>(resp).await
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
    ) -> Result<GeckoTerminalResponse<TokenInfo>, reqwest::Error> {
        let path = format!("/networks/{network}/tokens/{address}/info");
        let params = json!({});
        let resp = self.get(path, params).await?;
        self.format_response::<TokenInfo>(resp).await
    }

    /// Get most recently updated 100 tokens info from all networks.
    pub async fn token_info_recently_updated(
        &self,
    ) -> Result<GeckoTerminalResponse<Vec<TokenInfo>>, reqwest::Error> {
        let path = "/tokens/info_recently_updated".to_string();
        let include_str = "network";
        let params = json!({ "include": include_str });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<TokenInfo>>(resp).await
    }

    /// Get trades of a pool on a network.
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
    ) -> Result<GeckoTerminalResponse<Vec<Trade>>, reqwest::Error> {
        let path = format!("/networks/{network}/pools/{pool_address}/trades");
        let params =
            json!({ "trade_volume_in_usd_greater_than": trade_volume_in_usd_greater_than });
        let resp = self.get(path, params).await?;
        self.format_response::<Vec<Trade>>(resp).await
    }

    /// Fetches the OHLCV (Open, High, Low, Close, Volume) data for a specific pool on a network.
    ///
    /// # Arguments
    /// * `network` - The network ID of the network to get the OHLCV data for.
    /// * `pool_address` - The address of the pool to get the OHLCV data for.
    /// * `timeframe` - The timeframe for the OHLCV data. This can be "day", "hour", etc.
    /// * `aggregate` - The aggregate parameter for the OHLCV data. This is optional and defaults to 1.
    /// * `before_timestamp` - The timestamp before which the OHLCV data should be fetched. This is optional and defaults to the current timestamp.
    /// * `limit` - The limit on the number of OHLCV data points to fetch. This is optional and defaults to 100.
    /// * `currency` - The currency in which the OHLCV data should be fetched. This is optional and defaults to "usd".
    /// * `token` - The token for which the OHLCV data should be fetched. This is optional and defaults to "base".
    #[allow(clippy::too_many_arguments)]
    pub async fn network_pool_ohlcv(
        &self,
        network: &str,
        pool_address: &str,
        timeframe: &str,
        aggregate: Option<i32>,
        before_timestamp: Option<u64>,
        limit: Option<i32>,
        currency: Option<&str>,
        token: Option<&str>,
    ) -> Result<GeckoTerminalResponse<OHLCV>, reqwest::Error> {
        let aggregate = aggregate.unwrap_or(1);
        let before_timestamp = before_timestamp.unwrap_or(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        let limit = limit.unwrap_or(100);
        let currency = currency.unwrap_or("usd");
        let token = token.unwrap_or("base");

        check_timeframe(timeframe);
        check_aggregate(&aggregate, timeframe);
        check_ohlcv_limit(&limit);
        check_currency(currency);
        check_token(token);

        let path = format!("/networks/{network}/pools/{pool_address}/ohlcv/{timeframe}");
        let params = json!({ "aggregate": aggregate, "before_timestamp": before_timestamp, "limit": limit, "currency": currency, "token": token });
        let resp = self.get(path, params).await?;
        self.format_response::<OHLCV>(resp).await
    }
}

#[cfg(test)]
mod tests {
    use more_asserts::assert_gt;

    use super::*;

    #[tokio::test]
    async fn test_networks() {
        let client = GeckoTerminalAPI::new();
        let resp = client.networks(None).await.unwrap();
        assert_gt!(resp.data.len(), 10);
        assert_eq!(resp.data[0].type_field, "network");
    }

    #[tokio::test]
    async fn test_network_dexes() {
        let client = GeckoTerminalAPI::new();
        let resp = client.network_dexes("eth", None).await.unwrap();
        assert_gt!(resp.data.len(), 10);
        assert_eq!(resp.data[0].type_field, "dex");
    }

    #[tokio::test]
    async fn test_trending_pools() {
        let client = GeckoTerminalAPI::new();
        let resp = client.trending_pools(None).await.unwrap();
        assert_gt!(resp.data.len(), 3);
        assert_eq!(resp.data[0].type_field, "pool");
    }

    #[tokio::test]
    async fn test_network_trending_pools() {
        let client = GeckoTerminalAPI::new();
        let resp = client.network_trending_pools("eth", None).await.unwrap();
        assert_gt!(resp.data.len(), 3);
        assert_eq!(resp.data[0].type_field, "pool");
    }

    #[tokio::test]
    async fn test_network_pool_address() {
        let client = GeckoTerminalAPI::new();
        let resp = client
            .network_pool_address("eth", "0x60594a405d53811d3bc4766596efd80fd545a270")
            .await
            .unwrap();
        assert_eq!(
            resp.data.attributes.address,
            "0x60594a405d53811d3bc4766596efd80fd545a270"
        );
        assert_eq!(resp.data.type_field, "pool");
    }

    #[tokio::test]
    async fn test_network_pools_multi_address() {
        let client = GeckoTerminalAPI::new();
        let resp = client
            .network_pools_multi_address(
                "eth",
                vec![
                    "0x60594a405d53811d3bc4766596efd80fd545a270",
                    "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640",
                ],
            )
            .await
            .unwrap();
        assert_eq!(resp.data.len(), 2);
        assert_eq!(resp.data[0].type_field, "pool");
    }

    #[tokio::test]
    async fn test_network_pools() {
        let client = GeckoTerminalAPI::new();
        let resp = client.network_pools("eth", None).await.unwrap();
        assert_gt!(resp.data.len(), 10);
        assert_eq!(resp.data[0].type_field, "pool");
    }

    #[tokio::test]
    async fn test_network_dex_pools() {
        let client = GeckoTerminalAPI::new();
        let resp = client
            .network_dex_pools("eth", "sushiswap", None)
            .await
            .unwrap();
        assert_gt!(resp.data.len(), 10);
        assert_eq!(resp.data[0].type_field, "pool");
    }

    #[tokio::test]
    async fn test_network_new_pools() {
        let client = GeckoTerminalAPI::new();
        let resp = client.network_new_pools("eth", None).await.unwrap();
        assert_gt!(resp.data.len(), 10);
        assert_eq!(resp.data[0].type_field, "pool");
    }

    #[tokio::test]
    async fn test_new_pools() {
        let client = GeckoTerminalAPI::new();
        let resp = client.new_pools(None).await.unwrap();
        assert_gt!(resp.data.len(), 10);
        assert_eq!(resp.data[0].type_field, "pool");
    }

    #[tokio::test]
    async fn test_search_network_pool() {
        let client = GeckoTerminalAPI::new();
        let resp = client
            .search_network_pool("ETH", "eth", None)
            .await
            .unwrap();
        assert_gt!(resp.data.len(), 10);
        assert_eq!(resp.data[0].type_field, "pool");
    }

    #[tokio::test]
    async fn test_network_addresses_token_price() {
        let client = GeckoTerminalAPI::new();
        let resp = client
            .network_addresses_token_price(
                "eth",
                vec![
                    "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
                    "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
                ],
            )
            .await
            .unwrap();
        assert_eq!(resp.data.type_field, "simple_token_price");
    }

    #[tokio::test]
    async fn test_network_token_pools() {
        let client = GeckoTerminalAPI::new();
        let resp = client
            .network_token_pools("eth", "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", None)
            .await
            .unwrap();
        assert_gt!(resp.data.len(), 5);
        assert_eq!(resp.data[0].type_field, "pool");
    }

    #[tokio::test]
    async fn test_network_token() {
        let client = GeckoTerminalAPI::new();
        let resp = client
            .network_token("eth", "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")
            .await
            .unwrap();
        assert_eq!(
            resp.data.attributes.address,
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
        );
        assert_eq!(resp.data.type_field, "token");
    }

    #[tokio::test]
    async fn test_network_token_multi_address() {
        let client = GeckoTerminalAPI::new();
        let resp = client
            .network_token_multi_address(
                "eth",
                vec![
                    "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
                    "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
                ],
            )
            .await
            .unwrap();
        assert_eq!(resp.data.len(), 2);
        assert_eq!(resp.data[0].type_field, "token");
    }

    #[tokio::test]
    async fn test_network_tokens_address_info() {
        let client = GeckoTerminalAPI::new();
        let resp = client
            .network_tokens_address_info("eth", "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")
            .await
            .unwrap();
        assert_eq!(
            resp.data.attributes.address,
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
        );
        assert_eq!(resp.data.type_field, "token");
    }

    #[tokio::test]
    async fn test_token_info_recently_updated() {
        let client = GeckoTerminalAPI::new();
        let resp = client.token_info_recently_updated().await.unwrap();
        assert_gt!(resp.data.len(), 10);
        assert_eq!(resp.data[0].type_field, "token");
    }

    #[tokio::test]
    async fn test_network_pool_trades() {
        let client = GeckoTerminalAPI::new();
        let resp = client
            .network_pool_trades("eth", "0x60594a405d53811d3bc4766596efd80fd545a270", 1000.0)
            .await
            .unwrap();
        assert_gt!(resp.data.len(), 100);
        assert_eq!(resp.data[0].type_field, "trade");
    }

    #[tokio::test]
    async fn test_network_pool_ohlcv() {
        let client = GeckoTerminalAPI::new();
        let resp = client
            .network_pool_ohlcv(
                "eth",
                "0x60594a405d53811d3bc4766596efd80fd545a270",
                "day",
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(resp.data.type_field, "ohlcv_request_response");
        assert_eq!(resp.data.attributes.ohlcv_list.len(), 100);
    }

    #[tokio::test]
    async fn test_invalid_params() {
        let client = GeckoTerminalAPI::new();
        let resp = client.networks(Some(-1)).await;
        assert!(resp.is_err());
    }
}
