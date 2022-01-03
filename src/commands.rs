use crate::config_reader::CACHE_VIDEO_DIR;
use crate::util_macro::static_format;
use crate::web::yt_video::Video;
use std::path::Path;
use std::process::{Command, Stdio};

fn live_vid(url: &str) {
    Command::new("devour")
        .arg("mpv")
        .arg(url)
        .output()
        .expect("Failed");
}

pub fn open_in_br(url: &str) {
    Command::new("firefox")
        .arg(url)
        .spawn()
        .expect("filed to open");
}

pub fn yt_dl(url: &str) {
    let url = string_to_static_str(url.to_string());
    Command::new("yt-dlp")
        .arg("-o")
        .arg(static_format!("{}/%(id)s.%(ext)s", CACHE_VIDEO_DIR))
        .arg(url)
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to start");
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn play_vid(vid: &Video) {
    if Path::new(&static_format!("{}/{}.webm", CACHE_VIDEO_DIR, vid.id)).exists() {
        Command::new("devour")
            .arg("mpv")
            .arg(static_format!("{}/{}.webm", CACHE_VIDEO_DIR, vid.id))
            .output()
            .expect("failed to play video");
    } else {
        live_vid(&vid.clone().get_url());
    }
}
