#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = geckoterminal_rs::GeckoTerminalAPI::new();

    let _ = client.networks(1).await;

    let _ = client.network_dexes("eth", 1).await;

    let _ = client
        .trending_pools(vec!["base_token", "quote_token", "dex", "network"], 1)
        .await;

    let _ = client
        .network_trending_pools("eth", vec!["base_token", "quote_token", "dex"], 1)
        .await;

    let _ = client
        .network_pool_address(
            "eth",
            "0x60594a405d53811d3bc4766596efd80fd545a270",
            vec!["base_token", "quote_token", "dex"],
        )
        .await;

    let _ = client
        .network_pools_multi_address(
            "eth",
            vec![
                "0x60594a405d53811d3bc4766596efd80fd545a270",
                "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640",
            ],
            vec!["base_token", "quote_token", "dex"],
        )
        .await;

    let _ = client
        .network_pools("eth", vec!["base_token", "quote_token", "dex"], 1)
        .await;

    let _ = client
        .network_addresses_token_price(
            "eth",
            vec![
                "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
                "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
            ],
        )
        .await;

    let _ = client
        .network_token_pools(
            "eth",
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
            vec!["base_token", "quote_token", "dex"],
            1,
        )
        .await;

    let _ = client
        .network_token(
            "eth",
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
            vec!["top_pools"],
        )
        .await;

    let _ = client
        .network_token_multi_address(
            "eth",
            vec![
                "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
                "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
            ],
            vec!["top_pools"],
        )
        .await;

    let _ = client
        .network_tokens_address_info("eth", "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")
        .await;

    let _ = client
        .network_pool_trades("eth", "0x60594a405d53811d3bc4766596efd80fd545a270", 1000.0)
        .await;

    Ok(())
}
