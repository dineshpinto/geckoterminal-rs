use log::warn;

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