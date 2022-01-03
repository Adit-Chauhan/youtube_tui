use crate::config_reader::*;
use crate::list::*;
use crate::util_macro::*;
use crate::web::extra::{history, watch_later};
use crate::web::yt_video::Video;
use crate::web::*;
use ueberzug::{Scalers, UeConf, Ueberzug};

use std::io::Write;
use std::panic;

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

use super::{App, Contents, Menu};

impl App {
    #[allow(dead_code)]
    pub fn new(content: Contents) -> App {
        // Depreciated Kept in case i may need it later
        let vid_titles: Vec<String> = match &content {
            Contents::Vid(vi) => vi.iter().map(|v| -> String { v.title.clone() }).collect(),
            Contents::Chan(ch) => ch.iter().map(|c| -> String { c.name.clone() }).collect(),
        };
        App {
            titles: StatefulList::with_items(vid_titles),
            content,
            ueberzug: Ueberzug::new(),
            menu_titles: vec!["Home", "Recent", "Channels", "History", "Watch Later"],
            menu_active: Menu::Home,
        }
    }
    pub fn new_from_menu(m: Menu) -> App {
        App {
            titles: StatefulList::new(),
            content: Contents::Vid(Vec::new()),
            ueberzug: Ueberzug::new(),
            menu_titles: vec!["Home", "Recent", "Channels", "History", "Watch Later"],
            menu_active: m,
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

        let (content, titles) = match self.menu_active {
            Menu::Home => {
                let v = utils::get_home().expect("video");
                let t: Vec<String> = iter_collect!(v, |vv| -> String { vv.title.clone() });
                (Contents::Vid(v), StatefulList::with_items(t))
            }
            Menu::Recent => {
                let v = utils::get_recent().expect("video");
                let t: Vec<String> = iter_collect!(v, |vv| -> String { vv.title.clone() });
                (Contents::Vid(v), StatefulList::with_items(t))
            }
            Menu::Channels => {
                let c = utils::load_channels();
                let t = iter_collect!(c, |cc| -> String { cc.name.clone() });
                (Contents::Chan(c), StatefulList::with_items(t))
            }
            Menu::ChannelVideos => {
                let idx = self.titles.state.selected().unwrap_or(0);
                if let Contents::Chan(c) = &self.content {
                    let c = &c[idx];
                    let v = c.load_videos();
                    let t = iter_collect!(v, |vv| -> String { vv.title.clone() });
                    (Contents::Vid(v), StatefulList::with_items(t))
                } else {
                    panic!("")
                }
            }
            Menu::History => {
                let h = history::load_history();
                (Contents::Vid(h.0), StatefulList::with_items(h.1))
            }

            Menu::WatchList => {
                let w = watch_later::load_watch();
                (Contents::Vid(w.0), StatefulList::with_items(w.1))
            }
        };
        self.content = content;
        self.titles = titles;
    }

    pub fn return_img_path(&self, x: usize) -> String {
        if let Contents::Vid(videos) = &self.content {
            let filename = static_format!("{}/{}.jpg", CACHE_PATH, videos[x].id);
            if std::path::Path::new(&filename).exists() {
                return filename;
            } else {
                let url = videos[x].thumb_url.clone();
                // I am gonna keep this even though it may not be needed
                thread::spawn(|| {
                    download_thumb_idx(filename, url, false);
                    // kill the child when it has served it's purpose but dont print anything
                    //                   panic!("SILENT");
                });
                return static_format!("{}/loadingPath.png", PERMA);
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
            });
            return static_format!("{}/loadingPath.png", PERMA);
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
        if let Contents::Vid(videos) = &self.content {
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
        Menu::Home => app.draw_vids(bunks[1], f, "Home "),
        Menu::Recent => app.draw_vids(bunks[1], f, "Recents "),
        Menu::Channels => app.draw_channel(bunks[1], f),
        Menu::History => app.draw_simple(bunks[1], f, "History "),
        Menu::ChannelVideos => app.draw_vids(bunks[1], f, "Channel "),
        Menu::WatchList => app.draw_simple(bunks[1], f, "Watch Later "),
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
        .select(app.menu_active.as_num())
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
