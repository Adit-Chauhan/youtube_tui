mod config_reader;
mod list;
mod util_macro;

mod commands;
mod friendly_panic;
mod ui;
mod web;

use crate::config_reader::*;
use crate::friendly_panic::set_hooks as sh;
use crate::ui::{App, Menu};
use crate::web::extra::{cache, history};
use crate::web::utils::{get_channels, save_channel_vids, save_channels_initial, update_channels};

use log::info;
use std::error::Error;
use std::io::{self, Read};
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::style::Color::White;
use tui::{
    backend::TermionBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    //    sh();
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
    let mut asi = async_stdin();
    let stdout = io::stdout().into_raw_mode().expect("Raw IO");
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new_from_menu(unsafe { START_AT.clone() });
    app.refresh();
    terminal.clear()?;
    'outer: loop {
        terminal.draw(|f| crate::ui::utils::draw(f, &mut app))?;

        for k in asi.by_ref().keys() {
            match k.unwrap() {
                Key::Char('q') => {
                    // Clear the terminal before exit so as not to leave
                    // a mess.
                    terminal.clear()?;
                    break 'outer;
                }
                Key::Down => {
                    app.on_down();
                }
                Key::Up => {
                    app.on_up();
                }
                Key::Left => app.on_left(),
                Key::Right => app.on_right(),
                Key::Char('\t') => {
                    app.ueberzug.clear("a");
                    app.ueberzug.clear("1");
                    app.ueberzug.clear("2");
                    app.ueberzug.clear("3");
                    app.ueberzug.clear("4");
                    app.ueberzug.clear("0");
                    terminal.draw(|f| {
                        let para = Paragraph::new("Loading")
                            .style(tui::style::Style::default().fg(White))
                            .alignment(tui::layout::Alignment::Center)
                            .block(Block::default().borders(Borders::ALL));
                        f.render_widget(para, f.size());
                    })?;
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
