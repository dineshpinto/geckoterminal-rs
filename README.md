[![Crates.io Version](https://img.shields.io/crates/v/geckoterminal-rs)](https://crates.io/crates/geckoterminal-rs)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/geckoterminal-rs)](https://crates.io/crates/geckoterminal-rs)
[![Rust](https://github.com/dineshpinto/geckoterminal-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/dineshpinto/geckoterminal-rs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/dineshpinto/geckoterminal-rs/graph/badge.svg?token=OW7EKB0PXW)](https://codecov.io/gh/dineshpinto/geckoterminal-rs)
# geckoterminal-rs

## RESTful asynchronous Rust client for GeckoTerminal API

Wrapper around the [GeckoTerminal](https://www.geckoterminal.com) DeFi and DeX
aggregator operating across 90+ blockchains.

The API is currently in beta and is subject to change, please report any issues you
find.

## Installation

Your `Cargo.toml` could look like this:

```toml
[dependencies]
geckoterminal-rs = { version = "0.1" }
```

## Usage

```rust
use geckoterminal_rs::GeckoTerminalAPI;

#[tokio::main]
async fn main() {
    let client = GeckoTerminalAPI::new();

    // Get a list of supported networks
    let networks = client.networks(1).await;
    println!("{:?}", networks);

    // Query $ANALOS pool trades on Solana
    let trades = client.network_pool_trades("solana", "69grLw4PcSypZnn3xpsozCJFT8vs8WA5817VUVnzNGTh", 0).await;
    println!("{:?}", trades);

    // Query trending pools on Solana
    let pools = client.network_trending_pools("solana", vec!["base_token", "quote_token", "dex"], 0).await;
    println!("{:?}", pools);

    // And many more...
}
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
