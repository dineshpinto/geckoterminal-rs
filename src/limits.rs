pub const MAX_PAGE: i32 = 10;
pub const MAX_ADDRESSES: i32 = 30;

pub const POOL_INCLUDES: [&str; 4] = ["base_token", "quote_token", "dex", "network"];
pub const NETWORK_POOL_INCLUDES: [&str; 3] = ["base_token", "quote_token", "dex"];
pub const TOKEN_INCLUDES: [&str; 1] = ["top_pools"];
pub const TOKEN_INCLUDES_INFO: [&str; 1] = ["network"];

pub const TIMEFRAMES: [&str; 3] = ["day", "hour", "minute"];
pub const DAY_AGGREGATES: [i32; 1] = [1];
pub const HOUR_AGGREGATES: [i32; 3] = [1, 4, 12];
pub const MINUTE_AGGREGATES: [i32; 3] = [1, 5, 15];
pub const OHLCV_LIMIT: i32 = 1000;

pub const CURRENCIES: [&str; 2] = ["usd", "token"];
pub const TOKENS: [&str; 2] = ["base", "quote"];
