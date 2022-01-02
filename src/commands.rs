use std::process::Command;

pub fn play_vid(url: &str) {
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
