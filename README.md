![Crates.io Version](https://img.shields.io/crates/v/geckoterminal-rs)
![Crates.io Total Downloads](https://img.shields.io/crates/d/geckoterminal-rs)

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
use geckoterminal_rs::GeckoTerminal;

// Get a list of supported networks
#[tokio::main]
async fn main() {
    let client = GeckoTerminalAPI::new();
    let res = client.networks(1).await;
    println!("{:?}", res);
}
```
