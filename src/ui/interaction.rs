use crate::commands;
use crate::web::extra::history;
use crate::web::extra::watch_later;

use super::{App, Contents, Menu};
impl App {
    pub fn on_up(&mut self) {
        self.titles.previous();
    }
    pub fn on_down(&mut self) {
        self.titles.next();
    }
    pub fn on_right(&mut self) {}
    pub fn on_left(&mut self) {}
    pub fn on_tab(&mut self) {
        self.menu_active = self.menu_active.next();
        self.refresh();
    }
    pub fn on_enter(&mut self) {
        let idx = &self.titles.state;
        let idx = idx.selected().unwrap_or(0);

        match &self.content {
            Contents::Vid(x) => {
                history::save_history(&x[idx].title, &x[idx].id, &x[idx].channel);
                commands::play_vid(&x[idx]);
            }
            Contents::Chan(_x) => {
                self.menu_active = Menu::ChannelVideos;
                self.refresh();
            }
        };
    }
    pub fn on_char(&mut self, c: char) {
        if let Some(m) = Menu::get(c) {
            self.menu_active = m;
            self.refresh();
            return;
        }
        let idx = self.titles.state.selected().unwrap_or(0);
        match &self.content {
            Contents::Vid(vids) => {
                let idx = self.titles.state.selected().unwrap_or(0);
                if c == 'y' {
                    commands::open_in_br(&vids[idx].clone().get_url());
                    return;
                }
                if c == 'w' {
                    watch_later::save_watch(&vids[idx].title, &vids[idx].id, &vids[idx].channel);
                    return;
                }
                if c == 'W' {
                    watch_later::save_watch(&vids[idx].title, &vids[idx].id, &vids[idx].channel);
                    commands::yt_dl(&vids[idx].clone().get_url());
                }
            }
            Contents::Chan(chans) => {
                if c == 'y' {
                    let x = &chans[idx].id;
                    commands::open_in_br(&format!("https://www.youtube.com/channel/{}", x));
                    return;
                }
                let vids = chans[idx].load_videos();
                match c {
                    '1' => {
                        history::save_history(&vids[0].title, &vids[0].id, &vids[0].channel_url);
                        commands::play_vid(&vids[0])
                    }
                    '2' => {
                        history::save_history(&vids[1].title, &vids[1].id, &vids[1].channel_url);
                        commands::play_vid(&vids[1])
                    }
                    '3' => {
                        history::save_history(&vids[2].title, &vids[2].id, &vids[2].channel_url);
                        commands::play_vid(&vids[2])
                    }
                    '4' => {
                        history::save_history(&vids[3].title, &vids[3].id, &vids[3].channel_url);
                        commands::play_vid(&vids[3])
                    }
                    '5' => {
                        history::save_history(&vids[4].title, &vids[4].id, &vids[4].channel_url);
                        commands::play_vid(&vids[4])
                    }
                    _ => {}
                };
            }
        }
    }
}
