use crate::{
    list::*,
    web::{yt_channels::YTChannel, yt_video::Video},
};
use ueberzug::Ueberzug;
pub mod drawing;
pub mod interaction;
pub mod utils;

pub enum Contents {
    Vid(Vec<Video>),
    Chan(Vec<YTChannel>),
}

pub struct App {
    pub titles: StatefulList<String>,
    pub content: Contents,
    pub ueberzug: Ueberzug,
    pub menu_titles: Vec<&'static str>,
    pub menu_active: Menu,
}

pub enum Menu {
    Home,
    Recent,
    Channels,
    ChannelVideos,
    History,
    WatchList,
}
impl Menu {
    pub fn next(&self) -> Self {
        match self {
            Menu::Home => Menu::Recent,
            Menu::Recent => Menu::Channels,
            _ => Menu::Home,
        }
    }

    pub fn get(c: char) -> Option<Self> {
        match c {
            'h' => Some(Menu::Home),
            'r' => Some(Menu::Recent),
            'c' => Some(Menu::Channels),
            'p' => Some(Menu::History),
            'o' => Some(Menu::WatchList),
            _ => None,
        }
    }
    pub fn as_num(&self) -> usize {
        match self {
            Menu::Home => 0,
            Menu::Recent => 1,
            Menu::Channels => 2,
            Menu::History => 3,
            Menu::ChannelVideos => 5,
            Menu::WatchList => 4,
        }
    }
}
