use std::io::Read;

use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub cookie: String,
    pub cachepath: String,
    pub api_key: String,
    pub channel_id: String,
}

fn read_config() -> Config {
    let mut buf = "".to_string();
    let _a = std::fs::File::open("/home/adit/.config/yt.toml")
        .expect("failed to read config")
        .read_to_string(&mut buf);
    let rets: Config = toml::from_str(buf.as_str()).unwrap();
    rets
}
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn set_configs() {
    let a = read_config();
    unsafe {
        YT_API_KEY = string_to_static_str(a.api_key);
        YT_COOKIES = string_to_static_str(a.cookie);
        YT_SELF_CHANNEL_NAME = string_to_static_str(a.channel_id);
        CACHE_PATH = string_to_static_str(a.cachepath);
    }
}

pub static mut YT_COOKIES: &'static str = "";
pub static mut YT_API_KEY: &'static str = "";
pub static mut YT_SELF_CHANNEL_NAME: &'static str = "";
pub static mut CACHE_PATH: &'static str = "";
