mod api;
mod limits;
mod validation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = api::GeckoTerminalAPI::new();

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

    Ok(())
}
