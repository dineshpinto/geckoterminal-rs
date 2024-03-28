pub const MAX_PAGE: i32 = 10;
pub const MAX_ADDRESSES: usize = 30;
pub const OHLCV_LIMIT: i32 = 1000;

pub const VALID_TIMEFRAMES: [&str; 3] = ["day", "hour", "minute"];
pub const VALID_DAY_AGGREGATES: [i32; 1] = [1];
pub const VALID_HOUR_AGGREGATES: [i32; 3] = [1, 4, 12];
pub const VALID_MINUTE_AGGREGATES: [i32; 3] = [1, 5, 15];

pub const VALID_CURRENCIES: [&str; 2] = ["usd", "token"];
pub const VALID_TOKENS: [&str; 2] = ["base", "quote"];
