use log::{error, warn};

use crate::limits::{
    MAX_ADDRESSES,
    MAX_PAGE,
    NETWORK_POOL_INCLUDES,
    POOL_INCLUDES,
    TOKEN_INCLUDES,
    TOKEN_INCLUDES_INFO
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