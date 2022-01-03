pub mod history {

    macro_rules! panic {
        ($x:expr) => {
            std::panic::panic_any($x)
        };
    }

    use super::read_lines;
    use itertools::Itertools;
    use log::debug;
    use log::info;

    use crate::config_reader::*;
    use crate::util_macro::*;
    use crate::web::yt_video::Video;
    use std::fs;
    use std::io::Write;

    pub fn save_history(title: &str, id: &str, chan: &str) {
        let mut history = fs::OpenOptions::new()
            .append(true)
            .open(&static_format!("{}/history.txt", PERMA))
            .expect("file");
        errmap!(
            writeln!(history, "{}<>ID<>{}<>CHAN<>{}", title, id, chan),
            panic!(super::usr_panics::HISTORY_PANIC.clone())
        );
    }
    pub fn load_history() -> (Vec<Video>, Vec<String>) {
        if let Ok(lines) = read_lines(static_format!("{}/history.txt", PERMA)) {
            let mut history: Vec<Video> = iter_collect!(into lines,
            |line| {
                if let Ok(l) = line {
                    let parts = l.split("<>ID<>").collect::<Vec<_>>();
                    let parts2 = parts[1].split("<>CHAN<>").collect::<Vec<_>>();
                    Video::new_light(parts[0].to_string(),parts2[0].to_string(),parts2[1].to_string())
                } else {
                    Video::new_light("error".to_string(),"error".to_string(),"error".to_string())
                }
            });
            let mut vid_titles: Vec<String> =
                iter_collect!(history, |v| -> String { v.title.clone() });
            history.reverse();
            vid_titles.reverse();
            return (history, vid_titles);
        }
        (Vec::new(), Vec::new())
    }
    pub fn prune_history() {
        info!("Pruning Duplicates from History");
        let (hist, _) = load_history();
        let hist: Vec<&Video> = hist.iter().unique_by(|x| &x.id).collect();
        save_history_bulk(&hist);
    }

    fn save_history_bulk(hist: &Vec<&Video>) {
        debug!("Saving History File");
        let mut hist_file =
            fs::File::create(&static_format!("{}/history.txt", PERMA)).expect("File");
        let _ = hist
            .into_iter()
            .map(|entry| {
                errmap!(
                    writeln!(
                        hist_file,
                        "{}<>ID<>{}<>CHAN<>{}",
                        entry.title, entry.id, entry.channel
                    ),
                    panic!(super::usr_panics::HISTORY_PANIC.clone())
                );
            })
            .collect::<Vec<_>>();
    }
}

pub mod cache {
    use crate::config_reader::{CACHE_MAX_SIZE, CACHE_PATH};
    use fs_extra::dir;
    use log::info;
    use std::fs;
    pub fn prune_cache() {
        info!("Checking Cache");
        let size = dir::get_size(unsafe { CACHE_PATH }).unwrap();
        let size = size / 1_000_000;
        info!("Cache Size {} mb", size);
        if size as usize > unsafe { CACHE_MAX_SIZE } {
            info!("Deleting Cache");
            for path in fs::read_dir(unsafe { CACHE_PATH }).unwrap() {
                let path = path.unwrap().path();
                let ext = path.extension();
                if let Some(ext) = ext {
                    if ext == std::ffi::OsStr::new("jpg") {
                        fs::remove_file(path).unwrap();
                    }
                }
            }
        }
    }
}

pub mod watch_later {
    macro_rules! panic {
        ($x:expr) => {
            std::panic::panic_any($x)
        };
    }

    use super::read_lines;
    use super::usr_panics::WATCH_PANIC;
    use crate::config_reader::*;
    use crate::util_macro::*;
    use crate::web::yt_video::Video;

    use std::fs;
    use std::io::Write;
    pub fn save_watch(title: &str, id: &str, chan: &str) {
        let mut watch_list = fs::OpenOptions::new()
            .append(true)
            .open(&static_format!("{}/watch_list.txt", PERMA))
            .expect("file");
        errmap!(
            writeln!(watch_list, "{}<>ID<>{}<>CHAN<>{}", title, id, chan),
            panic!(WATCH_PANIC.clone())
        );
    }
    pub fn load_watch() -> (Vec<Video>, Vec<String>) {
        if let Ok(lines) = read_lines(static_format!("{}/watch_list.txt", PERMA)) {
            let mut watch_list: Vec<Video> = iter_collect!(into lines,
            |line| {
                if let Ok(l) = line {
                    let parts = l.split("<>ID<>").collect::<Vec<_>>();
                    let parts2 = parts[1].split("<>CHAN<>").collect::<Vec<_>>();
                    Video::new_light(parts[0].to_string(),parts2[0].to_string(),parts2[1].to_string())
                } else {
                    Video::new_light("error".to_string(),"error".to_string(),"error".to_string())
                }
            });
            let mut vid_titles: Vec<String> =
                iter_collect!(watch_list, |v| -> String { v.title.clone() });
            watch_list.reverse();
            vid_titles.reverse();
            return (watch_list, vid_titles);
        }
        (Vec::new(), Vec::new())
    }
    #[allow(dead_code)]
    fn save_watch_bulk(hist: &Vec<&Video>) {
        let mut hist_file =
            fs::File::create(&static_format!("{}/watch_list.txt", PERMA)).expect("File");
        let _ = hist
            .into_iter()
            .map(|entry| {
                errmap!(
                    writeln!(
                        hist_file,
                        "{}<>ID<>{}<>CHAN<>{}",
                        entry.title, entry.id, entry.channel
                    ),
                    panic!(WATCH_PANIC.clone())
                );
            })
            .collect::<Vec<_>>();
    }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub mod usr_panics {

    use crate::friendly_panic::{Instructions, UserPanic};
    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref HISTORY_PANIC: UserPanic = {
            let v = vec![
            Instructions{opener:"Check if the History File exists.\nNote: if The location for shared files is not specified then by default the file is in $HOME/.local/share/yt",instructions:Some(vec!["Simply Create a new file in the directory with the name of history.txt"])},
            Instructions{opener:"Check The permissions of the History file",instructions:Some(vec!["Go to the directory of the history file","Open the terminal and enter \"ls -l\"","make sure that permissions are in -rw-r--r-- format","you can do this by running chmod +rw history.txt"])}];
            UserPanic {
                error_msg: "The program failed to Open History File.",
                fix_instructions: Some(v),
            }
        };
    }
    lazy_static! {
        pub static ref WATCH_PANIC: UserPanic = {
            let v = vec![
            Instructions{opener:"Check if the watch_list.txt File exists.\nNote: if The location for shared files is not specified then by default the file is in $HOME/.local/share/yt",instructions:Some(vec!["Simply Create a new file in the directory with the name  watch_list.txt"])},
            Instructions{opener:"Check The permissions of the Watch List file",instructions:Some(vec!["Go to the directory of the watch file","Open the terminal and enter \"ls -l\"","make sure that permissions are in -rw-r--r-- format","you can do this by running chmod +rw watch_list.txt"])}];
            UserPanic {
                error_msg: "The program failed to Open Watch Later File.",
                fix_instructions: Some(v),
            }
        };
    }
}
