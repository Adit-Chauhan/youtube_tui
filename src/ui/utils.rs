use crate::config_reader::*;
use crate::list::*;
use crate::util_macro::*;
use crate::web::extra::history;
use crate::web::*;
use crate::web::{yt_channels::YTChannel, yt_video::Video};
use ueberzug::{Scalers, UeConf, Ueberzug};

use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::panic;
use std::path::Path;

use std::thread;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color::*, Modifier, Style},
    text::{Span, Spans},
    widgets::Tabs,
    widgets::{Block, Borders},
    Frame,
};

use super::{App, Contents};

impl App {
    pub fn new(videos: Contents) -> App {
        let vid_titles: Vec<String> = match &videos {
            Contents::Vid(vi) => vi.iter().map(|v| -> String { v.title.clone() }).collect(),
            Contents::Chan(ch) => ch.iter().map(|c| -> String { c.name.clone() }).collect(),
        };
        App {
            titles: StatefulList::with_items(vid_titles),
            videos: videos,
            ueberzug: Ueberzug::new(),
            menu_titles: vec!["Home", "Recent", "Channels", "History"],
            menu_active: 0,
        }
    }

    pub fn refresh(&mut self) {
        // Clear images
        self.ueberzug.clear("0");
        self.ueberzug.clear("1");
        self.ueberzug.clear("2");
        self.ueberzug.clear("3");
        self.ueberzug.clear("4");
        self.ueberzug.clear("a");

        if self.menu_active < 2 {
            // for Home and Recent
            let videos = match self.menu_active {
                1 => utils::get_recent().expect("videos"),
                0 => utils::get_home().expect("videos"),
                _ => panic!("How is this possible"),
            };
            let vid_titles: Vec<String> = iter_collect!(videos, |v| -> String { v.title.clone() });
            self.videos = Contents::Vid(videos);
            self.titles = StatefulList::with_items(vid_titles);
        } else if self.menu_active == 2 {
            // For channel List
            let channels = utils::load_channels();
            let titles: Vec<String> = iter_collect!(channels, |c| -> String { c.name.clone() });
            self.videos = Contents::Chan(channels);
            self.titles = StatefulList::with_items(titles);
        } else if self.menu_active == 3 {
            // For History
            let (history, vid_titles) = history::load_history();
            self.videos = Contents::Vid(history);
            self.titles = StatefulList::with_items(vid_titles);
        } else if self.menu_active == 4 {
            let idx = self.titles.state.selected().unwrap_or(0);
            let chan = match &self.videos {
                Contents::Chan(c) => c,
                Contents::Vid(_) => panic!("panic"),
            };
            let chan = &chan[idx];
            let vids = chan.load_videos();
            let vid_titles: Vec<String> = iter_collect!(vids, |v| -> String { v.title.clone() });
            self.videos = Contents::Vid(vids);
            self.titles = StatefulList::with_items(vid_titles);
        }
    }

    pub fn return_img_path(&self, x: usize) -> String {
        if let Contents::Vid(videos) = &self.videos {
            let filename = static_format!("{}/{}.jpg", CACHE_PATH, videos[x].id);
            if std::path::Path::new(&filename).exists() {
                return filename;
            } else {
                let url = videos[x].thumb_url.clone();
                // I am gonna keep this even though it may not be needed
                thread::spawn(|| {
                    download_thumb_idx(filename, url, false);
                    // Supress the panic Prints keeping terminal sane
                    panic::set_hook(Box::new(|_| {}));
                    // kill the child when it has served it's purpose
                    panic!("Done");
                });
                return static_format!("{}/loadingPath.png", CACHE_PATH);
            }
        }
        "".to_string()
    }

    pub fn thumb_from_video(&self, vid: &Video) -> String {
        let filename = static_format!("{}/{}.jpg", CACHE_PATH, vid.id);
        if std::path::Path::new(&filename).exists() {
            return filename;
        } else {
            let url = vid.thumb_url.clone();
            // I am gonna keep this even though it may not be needed
            thread::spawn(|| {
                download_thumb_idx(filename, url, false);
                // Supress the panic Prints keeping terminal sane
                panic::set_hook(Box::new(|_| {}));
                // kill the child when it has served it's purpose
                panic!("Done");
            });
            return static_format!("{}/loadingPath.png", CACHE_PATH);
        }
    }

    pub fn draw_video_thumb(&self, vid: &Video, chunk: &Rect, iden: &str) {
        let filename = self.thumb_from_video(vid);
        self.ueberzug.draw(&UeConf {
            identifier: iden,
            path: &filename,
            x: chunk.x + 2,
            y: chunk.y + 1,
            width: Some(chunk.width - 5),
            height: Some(chunk.height - 2),
            scaler: Some(Scalers::Cover),
            ..Default::default()
        });
    }

    pub fn disp_video(&mut self, x: usize, chumk: &Rect) {
        let filename = self.return_img_path(x);

        self.ueberzug.draw(&UeConf {
            identifier: "a",
            path: &filename,
            x: chumk.x + 1,
            y: chumk.y + 1,
            width: Some(chumk.width - 2),
            height: Some(chumk.height),
            scaler: Some(Scalers::FitContain),
            ..Default::default()
        });

        // Queue Next two images to download if present
        if let Contents::Vid(videos) = &self.videos {
            if x + 1 < videos.len() {
                let _ = self.return_img_path(x + 1);
            }
            if x + 2 < videos.len() {
                let _ = self.return_img_path(x + 2);
            }
        }
    }
}

pub fn download_thumb_idx(filename: String, url: String, force: bool) {
    if std::path::Path::new(&filename).exists() && !force {
        return;
    }
    if filename == "" || url == "" {
        return;
    }
    let resp = reqwest::blocking::get(url)
        .expect("failed img")
        .bytes()
        .expect("faiked getting");
    let mut a = std::fs::File::create(filename).expect("0");
    a.write_all(&resp).expect("failed writing");
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let bunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(f.size());

    match app.menu_active {
        0 => app.draw_vids(bunks[1], f, "Home "),
        1 => app.draw_vids(bunks[1], f, "Recents "),
        2 => app.draw_channel(bunks[1], f),
        3 => app.draw_history(bunks[1], f),
        4 => app.draw_vids(bunks[1], f, "Channel "),
        _ => panic!("panic"),
    }

    let menu = iter_collect!(app.menu_titles, |t| -> Spans {
        let (first, rest) = t.split_at(1);
        Spans::from(vec![
            Span::styled(
                first,
                Style::default()
                    .fg(Yellow)
                    .add_modifier(Modifier::UNDERLINED),
            ),
            Span::styled(rest, Style::default().fg(White)),
        ])
    });

    let tabs = Tabs::new(menu)
        .select(app.menu_active)
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(White))
        .highlight_style(Style::default().fg(Yellow))
        .divider(Span::raw("|"));
    f.render_widget(tabs, bunks[0]);
}

pub fn video_desc(v: &Video) -> String {
    format!(
        "Title: {}\nView Count: {}\nPosted Time: {}\nChannel: {}",
        v.title, v.view_count, v.posted_time, v.channel
    )
}
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
