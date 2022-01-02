#![feature(generators, generator_trait)]

mod config_reader;
mod list;
mod util_macro;

mod commands;
mod ui;
mod web;

use crate::config_reader::*;
use crate::web::extra::{cache, history};
use crate::web::utils::{
    get_channels, get_home, save_channel_vids, save_channels_initial, update_channels,
};

use crate::ui::{App, Contents};

use std::error::Error;
use std::io::{self, Read};
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::style::Color::White;
use tui::{
    backend::TermionBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use log::info;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    set_configs();
    let ar: Vec<String> = std::env::args().collect();
    if ar.len() == 2 {
        match ar[1].as_str() {
            "update" => {
                update_channels();
                history::prune_history();
                cache::prune_cache();
            }
            "init" => {
                info!("Starting Initilizations");
                let chans = get_channels()?;
                save_channels_initial(&chans);
                save_channel_vids();
            }
            _ => {}
        }
        return Ok(());
    }
    let res = get_home()?;
    info!("Downloaded and Parsed Video info");
    let mut asi = async_stdin();
    let stdout = io::stdout().into_raw_mode().expect("Raw IO");
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Terminal");
    let mut app = App::new(Contents::Vid(res));
    terminal.clear()?;
    'outer: loop {
        terminal
            .draw(|f| crate::ui::utils::draw(f, &mut app))
            .expect("naj");

        for k in asi.by_ref().keys() {
            match k.unwrap() {
                Key::Char('q') => {
                    // Clear the terminal before exit so as not to leave
                    // a mess.
                    terminal.clear().expect("m");
                    break 'outer;
                }
                Key::Down => {
                    app.on_move(true);
                }
                Key::Up => {
                    app.on_move(false);
                }
                Key::Left => app.on_left(),
                Key::Right => app.on_right(),
                Key::Char('\t') => {
                    app.ueberzug.clear("a");
                    terminal.draw(|f| {
                        let para = Paragraph::new("Loading")
                            .style(tui::style::Style::default().fg(White))
                            .alignment(tui::layout::Alignment::Center)
                            .block(Block::default().borders(Borders::ALL));
                        f.render_widget(para, f.size());
                    });
                    app.on_tab();
                }
                Key::Char('\n') => app.on_enter(),
                Key::Char(c) => app.on_char(c),
                _ => (),
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(33));
    }

    Ok(())
}
