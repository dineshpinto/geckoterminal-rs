mod api;

#[tokio::main]
async fn main() {
    let client = api::GeckoTerminalAPI::new();

    let res = client.networks("1").await;
    println!("{:?}", res);

    let res = client.network_dexes("eth", "1").await;
    println!("{:?}", res);

    let include = vec!["base_token", "quote_token", "dex", "network"];
    let res = client.trending_pools(include, "1").await;
    println!("{:?}", res);
}