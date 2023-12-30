use log::{error, warn};

use crate::limits::{
    CURRENCIES,
    DAY_AGGREGATES,
    HOUR_AGGREGATES,
    MAX_ADDRESSES,
    MAX_PAGE,
    MINUTE_AGGREGATES,
    NETWORK_POOL_INCLUDES,
    OHLCV_LIMIT,
    POOL_INCLUDES,
    TIMEFRAMES,
    TOKEN_INCLUDES,
    TOKEN_INCLUDES_INFO,
    TOKENS
};

pub fn check_page(page: &i32) {
    if page > &MAX_PAGE {
        warn!("page must be less than {}", MAX_PAGE)
    }
}

pub fn check_addresses(addresses: &Vec<&str>) {
    if addresses.len() as i32 > MAX_ADDRESSES {
        warn!("addresses must be less than {}", MAX_ADDRESSES)
    }
}

pub fn check_include(include: &Vec<&str>, include_type: &str) {
    match include_type {
        "pool" => {
            for i in include {
                if !POOL_INCLUDES.contains(i) {
                    warn!("{} not in {:?}", i, POOL_INCLUDES)
                }
            }
        },
        "network_pool" => {
            for i in include {
                if !NETWORK_POOL_INCLUDES.contains(i) {
                    warn!("{} not in {:?}", i, NETWORK_POOL_INCLUDES)
                }
            }
        },
        "token" => {
            for i in include {
                if !TOKEN_INCLUDES.contains(i) {
                    warn!("{} not in {:?}", i, TOKEN_INCLUDES)
                }
            }
        },
        "token_info" => {
            for i in include {
                if !TOKEN_INCLUDES_INFO.contains(i) {
                    warn!("{} not in {:?}", i, TOKEN_INCLUDES_INFO)
                }
            }
        },
        _ => error!("invalid include type {}", include_type)
    };
}

pub fn check_timeframe(timeframe: &str) {
    if !TIMEFRAMES.contains(&timeframe) {
        warn!("timeframe not in {:?}", TIMEFRAMES)
    }
}

pub fn check_aggregate(aggregate: &i32, timeframe: &str) {
    match timeframe {
        "day" => {
            if !DAY_AGGREGATES.contains(aggregate) {
                warn!("aggregate not in {:?}", DAY_AGGREGATES)
            }
        },
        "hour" => {
            if !HOUR_AGGREGATES.contains(aggregate) {
                warn!("aggregate not in {:?}", HOUR_AGGREGATES)
            }
        },
        "minute" => {
            if !MINUTE_AGGREGATES.contains(aggregate) {
                warn!("aggregate not in {:?}", MINUTE_AGGREGATES)
            }
        },
        _ => error!("invalid timeframe {}", timeframe)
    };
}

pub fn check_ohlcv_limit(limit: &i32) {
    if limit > &OHLCV_LIMIT {
        warn!("limit must be less than {}", OHLCV_LIMIT)
    }
}

pub fn check_currency(currency: &str) {
    if !CURRENCIES.contains(&currency) {
        warn!("currency not in {:?}", CURRENCIES)
    }
}

pub fn check_token(token: &str) {
    if !TOKENS.contains(&token) {
        warn!("token not in {:?}", TOKENS)
    }
}