use daemonize::Daemonize;
use std::fs::File;
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

pub fn yt_dl(url: &str) {
    let url = string_to_static_str(url.to_string());
    let h = std::thread::spawn(move || {
        let stdout = File::create("/tmp/daemon.out").unwrap();
        let stderr = File::create("/tmp/daemon.err").unwrap();

        let daemonize = Daemonize::new()
            .pid_file("/tmp/test.pid") // Every method except `new` and `start`
            .chown_pid_file(true) // is optional, see `Daemonize` documentation
            .working_directory("/tmp") // for default behaviour.
            .user("nobody")
            .group("daemon") // Group name
            .group(2) // or group id.
            .umask(0o777) // Set umask, `0o027` by default.
            .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
            .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
            .privileged_action(|| "Executed before drop privileges");

        match daemonize.start() {
            Ok(_) => println!("Success, daemonized"),
            Err(e) => eprintln!("Error, {}", e),
        }
        Command::new("yt-dlp")
            .arg("-o")
            .arg("\"%(id)s.%(ext)s\"")
            .arg(url)
            .spawn()
            .expect("failed to start");
    });
    h.join();
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
