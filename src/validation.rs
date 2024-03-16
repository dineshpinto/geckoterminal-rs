use crate::limits::{
    CURRENCIES, DAY_AGGREGATES, HOUR_AGGREGATES, MAX_ADDRESSES, MAX_PAGE, MINUTE_AGGREGATES,
    OHLCV_LIMIT, TIMEFRAMES, TOKENS,
};

pub fn check_page(page: &i32) {
    if page > &MAX_PAGE {
        log::warn!("page must be less than {}", MAX_PAGE)
    }
}

pub fn check_addresses(addresses: &[&str]) {
    if addresses.len() as i32 > MAX_ADDRESSES {
        log::warn!("addresses must be less than {}", MAX_ADDRESSES)
    }
}

pub fn check_timeframe(timeframe: &str) {
    if !TIMEFRAMES.contains(&timeframe) {
        log::warn!("timeframe not in {:?}", TIMEFRAMES)
    }
}

pub fn check_aggregate(aggregate: &i32, timeframe: &str) {
    match timeframe {
        "day" => {
            if !DAY_AGGREGATES.contains(aggregate) {
                log::warn!("aggregate not in {:?}", DAY_AGGREGATES)
            }
        }
        "hour" => {
            if !HOUR_AGGREGATES.contains(aggregate) {
                log::warn!("aggregate not in {:?}", HOUR_AGGREGATES)
            }
        }
        "minute" => {
            if !MINUTE_AGGREGATES.contains(aggregate) {
                log::warn!("aggregate not in {:?}", MINUTE_AGGREGATES)
            }
        }
        _ => log::error!("invalid timeframe {}", timeframe),
    };
}

pub fn check_ohlcv_limit(limit: &i32) {
    if limit > &OHLCV_LIMIT {
        log::warn!("limit must be less than {}", OHLCV_LIMIT)
    }
}

pub fn check_currency(currency: &str) {
    if !CURRENCIES.contains(&currency) {
        log::warn!("currency not in {:?}", CURRENCIES)
    }
}

pub fn check_token(token: &str) {
    if !TOKENS.contains(&token) {
        log::warn!("token not in {:?}", TOKENS)
    }
}
