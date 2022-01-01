pub mod history {
    use itertools::Itertools;
    use log::debug;
    use log::info;

    use crate::config_reader::*;
    use crate::util_macro::*;
    use crate::web::yt_video::Video;
    use std::fs;
    use std::fs::File;
    use std::io::{self, BufRead, Write};
    use std::path::Path;

    pub fn save_history(title: &str, id: &str, chan: &str) {
        let mut history = fs::OpenOptions::new()
            .append(true)
            .open(&static_format!("{}/history.txt", CACHE_PATH))
            .expect("file");
        writeln!(history, "{}<>ID<>{}<>CHAN<>{}", title, id, chan);
    }
    pub fn load_history() -> (Vec<Video>, Vec<String>) {
        if let Ok(lines) = read_lines(static_format!("{}/history.txt", CACHE_PATH)) {
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
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
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
            fs::File::create(&static_format!("{}/history.txt", CACHE_PATH)).expect("File");
        hist.into_iter()
            .map(|entry| {
                writeln!(
                    hist_file,
                    "{}<>ID<>{}<>CHAN<>{}",
                    entry.title, entry.id, entry.channel
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
