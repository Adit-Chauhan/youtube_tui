use log::info;
use serde::Deserialize;
use std::env;
use std::io::Read;
use toml;

#[derive(Deserialize, Debug)]
struct Config {
    cookie: String,
    api_key: String,
    channel_id: String,
    cachepath: Option<String>,
    cache_size_max: Option<usize>,
    video_dir: Option<String>,
    permapath: Option<String>,
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
    let home = env::var("HOME");
    unsafe {
        YT_API_KEY = string_to_static_str(a.api_key);
        YT_COOKIES = string_to_static_str(a.cookie);
        YT_SELF_CHANNEL_NAME = string_to_static_str(a.channel_id);
        CACHE_PATH = string_to_static_str(a.cachepath.unwrap_or(format!(
            "{}/.cache/yt",
            home.as_ref().expect("HOME Variable not set")
        )));
        CACHE_MAX_SIZE = a.cache_size_max.unwrap_or(64);
        CACHE_VIDEO_DIR = string_to_static_str(a.video_dir.unwrap_or(format!(
            "{}/.local/share/yt/videos",
            home.as_ref().expect("HOME Variable not set")
        )));
        PERMA = string_to_static_str(a.permapath.unwrap_or(format!(
            "{}/.local/share/yt/",
            home.as_ref().expect("HOME Variable not set")
        )));
        info!("API = {}", YT_API_KEY);
        info!("COOKIES = {}", YT_COOKIES);
        info!("CHANNEL = {}", YT_SELF_CHANNEL_NAME);
        info!("CACHE PATH = {}", CACHE_PATH);
        info!("CACHE SIZE = {}", CACHE_MAX_SIZE);
        info!("Video Dir = {}", CACHE_VIDEO_DIR);
        info!("PERMA = {}", PERMA);
    }
}

pub static mut YT_COOKIES: &'static str = "";
pub static mut YT_API_KEY: &'static str = "";
pub static mut YT_SELF_CHANNEL_NAME: &'static str = "";
pub static mut CACHE_PATH: &'static str = "";
pub static mut CACHE_MAX_SIZE: usize = 0;
pub static mut CACHE_VIDEO_DIR: &'static str = "";
pub static mut PERMA: &'static str = "";
