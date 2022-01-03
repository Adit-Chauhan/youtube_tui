use std::io::Read;

use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug)]
struct Config {
    cookie: String,
    cachepath: String,
    api_key: String,
    channel_id: String,
    cache_size_max: usize,
    video_dir: String,
    permapath: String,
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
        CACHE_MAX_SIZE = a.cache_size_max;
        CACHE_VIDEO_DIR = string_to_static_str(a.video_dir);
        PERMA = string_to_static_str(a.permapath);
    }
}

pub static mut YT_COOKIES: &'static str = "";
pub static mut YT_API_KEY: &'static str = "";
pub static mut YT_SELF_CHANNEL_NAME: &'static str = "";
pub static mut CACHE_PATH: &'static str = "";
pub static mut CACHE_MAX_SIZE: usize = 0;
pub static mut CACHE_VIDEO_DIR: &'static str = "";
pub static mut PERMA: &'static str = "";
