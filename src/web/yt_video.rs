use std::path::Path;

use crate::web::api as YTApi;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub thumb_url: String,
    pub view_count: String,
    pub posted_time: String,
    pub channel: String,
    pub channel_url: String,
}

#[derive(Debug, Clone)]
pub struct Comment {
    author: String,
    comment: String,
}

impl Video {
    pub(super) fn new(
        id: String,
        title: String,
        thumb_url: String,
        view_count: String,
        posted_time: String,
        channel: String,
        channel_url: String,
    ) -> Self {
        Self {
            id,
            title,
            thumb_url,
            view_count,
            posted_time,
            channel,
            channel_url,
        }
    }

    pub fn new_light(title: String, id: String, chan: String) -> Self {
        Self {
            id: id,
            title: title,
            thumb_url: String::new(),
            view_count: String::new(),
            posted_time: String::new(),
            channel: chan,
            channel_url: String::new(),
        }
    }

    pub fn get_url(self) -> String {
        format!("https://www.youtube.com/watch?v={}", self.id)
    }
    pub fn get_thumb_loc(self) -> String {
        if Path::new(&format!("{}.jpg", self.id)).exists() {
            format!("{}.jpg", self.id)
        } else {
            "loadingPath.pg".to_string()
        }
    }
}
