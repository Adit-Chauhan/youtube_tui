use crate::util_macro::*;
use crate::web::extra::history;
use std::panic;
use std::process::Command;

const MENU_COUNT: usize = 4;
use super::{App, Contents};
impl App {
    pub fn on_move(&mut self, di: bool) {
        tern![di => self.titles.next();self.titles.previous()];
    }
    pub fn on_right(&mut self) {}
    pub fn on_left(&mut self) {}
    pub fn on_tab(&mut self) {
        self.menu_active = (self.menu_active + 1) % MENU_COUNT;
        self.refresh();
    }
    pub fn on_enter(&mut self) {
        let idx = &self.titles.state;
        let idx = idx.selected().unwrap_or(0);

        match &self.videos {
            Contents::Vid(x) => {
                history::save_history(&x[idx].title, &x[idx].id, &x[idx].channel);
                Command::new("devour")
                    .arg("mpv")
                    .arg(x[idx].clone().get_url())
                    .output()
                    .expect("Failed");
            }
            Contents::Chan(_x) => {
                self.menu_active = 4;
                self.refresh();
            }
        };
    }
    pub fn on_char(&mut self, c: char) {
        match c {
            'h' => {
                self.menu_active = 0;
                self.refresh();
            }
            'r' => {
                self.menu_active = 1;
                self.refresh();
            }
            'c' => {
                self.menu_active = 2;
                self.refresh();
            }
            'p' => {
                self.menu_active = 3;
                self.refresh();
            }
            _ => {}
        }

        if self.menu_active == 2 {
            let run_vid = |url: String| {
                Command::new("devour")
                    .arg("mpv")
                    .arg(url)
                    .output()
                    .expect("failed");
            };

            let idx = self.titles.state.selected().unwrap_or(0);
            let chans = match &self.videos {
                Contents::Chan(x) => x,
                _ => panic!("Eror"),
            };
            let vids = chans[idx].load_videos();
            // TODO Fix Channel Name and url mixing
            match c {
                '1' => {
                    history::save_history(&vids[0].title, &vids[0].id, &vids[0].channel_url);
                    run_vid(vids[0].clone().get_url())
                }
                '2' => {
                    history::save_history(&vids[1].title, &vids[1].id, &vids[1].channel_url);
                    run_vid(vids[1].clone().get_url())
                }
                '3' => {
                    history::save_history(&vids[2].title, &vids[2].id, &vids[2].channel_url);
                    run_vid(vids[2].clone().get_url())
                }
                '4' => {
                    history::save_history(&vids[3].title, &vids[3].id, &vids[3].channel_url);
                    run_vid(vids[3].clone().get_url())
                }
                '5' => {
                    history::save_history(&vids[4].title, &vids[4].id, &vids[4].channel_url);
                    run_vid(vids[4].clone().get_url())
                }
                _ => {}
            };
        }
    }
}
