pub mod api;
pub mod utils;

use std::fs;
use std::io::Write;
use std::path::Path;

use crate::config_reader::*;
use crate::util_macro::*;
use crate::web::api::YTApi;

use log::info;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub thumb_url: String,
    pub view_count: String,
    pub posted_time: String,
    pub channel: String,
    pub channel_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YTChannel {
    pub name: String,
    pub id: String,
    pub description: String,
    pub thumbnail_url: String,
    pub item_count: String,
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

impl YTChannel {
    pub fn new(
        name: String,
        id: String,
        description: String,
        thumbnail_url: String,
        item_count: String,
    ) -> Self {
        Self {
            name,
            id,
            description,
            thumbnail_url,
            item_count,
        }
    }
    pub(super) fn save_channel(&self) {
        let foldername = &static_format!("{}/{}", CACHE_PATH, self.id);
        let filename = &static_format!("{}/{}/{}.json", CACHE_PATH, self.id, self.id);
        if !Path::new(foldername).is_dir() {
            fs::create_dir_all(foldername).expect("couldnot create folder");
        }
        json_file!(write self,filename);
    }

    pub fn get_videos(&self, _full: bool) -> Vec<Video> {
        println!("Grabbing video for {}", self.name);
        let jsn = YTApi::new()
            .get_channel_uploads(self.id.to_string(), None)
            .unwrap();
        let jsn: Value = serde_json::from_str(&jsn).unwrap();
        let vids = jsn["items"].as_array().unwrap();
        let vids: Vec<Video> = iter_collect!(vids, |v| {
            info!("{}", vid_travel!(v, "snippet", "thumbnails", "high", "url"));
            Video::new(
                vid_travel!(v, "contentDetails", "videoId"),
                vid_travel!(v, "snippet", "title"),
                vid_travel!(v, "snippet", "thumbnails", "high", "url"),
                "".to_string(),
                vid_travel!(v, "snippet", "publishedAt"),
                vid_travel!(v, "snippet", "channelTitle"),
                vid_travel!(v, "snippet", "channelId"),
            )
        });
        vids
    }
    pub fn save_vidoes(&self, base_loc: String) {
        let vids = self.get_videos(true);
        json_file!(write & vids, &format!("{}_vids.json", base_loc));
    }
    pub fn load_videos(&self, base_loc: String) -> Vec<Video> {
        let json = fs::read_to_string(&format!("{}_vids.json", base_loc)).expect("");
        let json: Vec<Video> = serde_json::from_str(&json).expect("");
        json
    }
}
