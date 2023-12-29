use log::warn;

use crate::limits::{NETWORK_POOL_INCLUDES, POOL_INCLUDES, TOKEN_INCLUDES, TOKEN_INCLUDES_INFO};

pub fn check_page(page: &i32, max_page: i32) {
    if page > &max_page {
        warn!("page must be less than {}", max_page)
    }
}

pub fn check_addresses(addresses: &Vec<&str>, max_addresses: i32) {
    if addresses.len() as i32 > max_addresses {
        warn!("addresses must be less than {}", max_addresses)
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
        _ => panic!("invalid include type")
    };
}