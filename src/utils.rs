use std::error::Error;

use cfg_if::cfg_if;
use worker::*;
use regex::Regex;

cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

pub fn extract_products(input: &str) -> std::result::Result<&str, Box<dyn Error>> {
    Ok(Regex::new(r"var products = (.*?);")?
        .captures(input).ok_or("Regex failed to find the products line")?
        .get(1).ok_or("Regex failed to find the products array")?
        .as_str())
}
