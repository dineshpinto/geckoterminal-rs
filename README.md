[![Crates.io Version](https://img.shields.io/crates/v/geckoterminal-rs)](https://crates.io/crates/geckoterminal-rs)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/geckoterminal-rs)](https://crates.io/crates/geckoterminal-rs)
[![Rust](https://github.com/dineshpinto/geckoterminal-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/dineshpinto/geckoterminal-rs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/dineshpinto/geckoterminal-rs/graph/badge.svg?token=OW7EKB0PXW)](https://codecov.io/gh/dineshpinto/geckoterminal-rs)

# geckoterminal-rs

## RESTful asynchronous Rust client for GeckoTerminal API

Wrapper around the [GeckoTerminal](https://www.geckoterminal.com) DeFi and DeX
aggregator operating across 90+ chains and 500+ dexes.

Features:

- Get the market data (price, volume, historical chart) of any token
- Find all the pools that are trading a specific token
- Plot a candlestick chart using OHLCV when given a pool address

The API is currently in beta and is subject to change, please report any issues you
find.

## Installation

```bash
cargo add geckoterminal-rs
```

or via `Cargo.toml`:

```toml
[dependencies]
geckoterminal-rs = "0.2"
```

## Docs

See the [GeckoTerminal API docs](https://www.geckoterminal.com/dex-api) for more
details.

## Usage

```rust
use geckoterminal_rs::GeckoTerminalAPI;

#[tokio::main]
async fn main() {
    let gt = GeckoTerminalAPI::new();

    // Get a list of supported networks
    let networks = gt.networks(1).await.unwrap();
    println!("{:?}", networks);

    // Query trending pools on a network
    let pools = gt.network_trending_pools("solana", 1).await.unwrap();
    println!("{:?}", pools);

    // and many more...
}
```

## Examples

### Get a list of supported networks

```rust
let networks = gt.networks(1).await.unwrap();
for network in networks.data {
    println!(
        "{} ({}, {})",
        network.attributes.name, network.id,
        network.attributes.coingecko_asset_platform_id.unwrap_or("".to_string())
    );
}
```

Output:

```text
Ethereum: (eth, ethereum)
Solana: (solana, solana)
Arbitrum: (arbitrum, arbitrum-one)
Polygon POS: (polygon_pos, polygon-pos)
Avalanche: (avax, avalanche)
Mantle: (mantle, mantle)
...
```

### Get list of trending pools on a network

```rust
let trending = gt.network_trending_pools("solana", 1).await.unwrap();
for pool in trending.data {
    println!(
        "{}: ${} (24H: {}%), Vol: {}, {} ({})",
        pool.attributes.name, pool.attributes.base_token_price_usd,
        pool.attributes.price_change_percentage.h24, pool.attributes.volume_usd.h24,
        pool.attributes.address, pool.relationships.dex.data.id
    );
}
```

Output:

```text
MONKEY / SOL: $0.0000000021245208116085 (24H: 181.76%), Vol: 315625.360191837, Dqb7bL7MZkuDgHrZZphRMRViJnepHxf9odx3RRwmifur (raydium)
BOME / SOL: $0.0216091492330017 (24H: 359.24%), Vol: 483264168.055883, DSUvc5qf5LJHHV5e2tD184ixotSnCnwj7i4jJa4Xsrmt (raydium)
BONK2.0 / RAY: $0.0000000971470021753641 (24H: 84.85%), Vol: 813080.861892264, 3YbvsPC4arDfhY4VLNXqtf3wRPAUUe7NxFmeZykUnHoZ (raydium)
PORTNOY / SOL: $0.000823561633783244 (24H: -68.9%), Vol: 1450271.6596655, 77JrcxAzPUEvn9o1YXmFm9zQid8etT4SCWVxVqE8VTTG (raydium)
PENG / SOL: $0.958285437444166 (24H: -34.31%), Vol: 13181611.9610447, AxBDdiMK9hRPLMPM7k6nCPC1gRARgXQHNejfP2LvNGr6 (raydium)
NINJA / SOL: $0.014017222386801 (24H: 0.85%), Vol: 2734212.42346667, B8sv1wiDf9VydktLmBDHUn4W5UFkFAcbgu7igK6Fn2sW (raydium)
IQ50 / SOL: $0.000091945701104404 (24H: 77.65%), Vol: 31955796.5143891, AFCHx2PfKB8Szn6196kcrHtbwP7zmi5ijMcJfurq1owv (orca)
boden / SOL: $0.115666833387918 (24H: -25.42%), Vol: 4656209.62137547, 6UYbX1x8YUcFj8YstPYiZByG7uQzAq2s46ZWphUMkjg5 (raydium)
$WIF / SOL: $2.58524513809515 (24H: -7.83%), Vol: 25117725.2604335, EP2ib6dYdEeqD8MfE2ezHCxX3kP3K2eLKkirfPm5eyMx (raydium)
...
```

## Disclaimer

This project is for educational purposes only. You should not construe any such
information or other material as legal, tax, investment, financial, or other advice.
Nothing contained here constitutes a solicitation, recommendation, endorsement, or
offer by me or any third party service provider to buy or sell any securities or other
financial instruments in this or in any other jurisdiction in which such solicitation or
offer would be unlawful under the securities laws of such jurisdiction.

Under no circumstances will I be held responsible or liable in any way for any claims,
damages, losses, expenses, costs, or liabilities whatsoever, including, without
limitation, any direct or indirect damages for loss of profits.
