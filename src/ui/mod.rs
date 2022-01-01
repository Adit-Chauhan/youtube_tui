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
    pub videos: Contents,
    pub ueberzug: Ueberzug,
    pub menu_titles: Vec<&'static str>,
    pub menu_active: usize,
}
