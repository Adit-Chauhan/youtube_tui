use std::fs;
use std::io::Write;
use std::path::Path;

use crate::config_reader::*;
use crate::util_macro::*;
use crate::web::api as YTApi;
use crate::web::yt_video::Video;

use itertools::Itertools;
use log::error;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YTChannel {
    pub name: String,
    pub id: String,
    pub description: String,
    pub thumbnail_url: String,
    pub item_count: String,
    pub upload_playlist: String,
    page_token: (String, String),
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
            upload_playlist: YTApi::get_channel_uploads_id(id.clone()).unwrap_or("".to_string()),
            name,
            id,
            description,
            thumbnail_url,
            item_count,
            page_token: (String::new(), String::new()),
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

    pub fn save_channel_initial(&self) {
        let vid = self.get_videos(true);
        self.save_channel();
        self.save_vidoes(&vid);
    }

    pub fn get_videos(&self, full: bool) -> Vec<Video> {
        let get_vid = |v: &Value| {
            //info!("{}", vid_travel!(v, "snippet", "thumbnails", "high", "url"));
            Video::new(
                vid_travel!(v, "contentDetails", "videoId"),
                vid_travel!(v, "snippet", "title"),
                vid_travel!(v, "snippet", "thumbnails", "high", "url"),
                "".to_string(),
                vid_travel!(v, "snippet", "publishedAt"),
                vid_travel!(v, "snippet", "channelTitle"),
                vid_travel!(v, "snippet", "channelId"),
            )
        };
        if full {
            info!("Grabbing all videos in {}", self.name);
            let mut vids = Vec::<Video>::new();
            let mut token = None;
            loop {
                info!("Using Page Token {:#?} for channel {} ", token, self.name);
                let jsn = YTApi::get_channel_uploads(self.id.to_string(), token).unwrap();
                let jsn: Value = serde_json::from_str(&jsn).unwrap();
                let v = jsn["items"].as_array().unwrap();
                let v: Vec<Video> = iter_collect!(v, get_vid);
                vids.extend(v);
                if let Some(a) = jsn["nextPageToken"].as_str() {
                    token = Some(a.to_string());
                } else {
                    break;
                }
            }
            return vids;
        }
        info!("Grabbing video for {}", self.name);
        let jsn = YTApi::get_channel_uploads(self.id.to_string(), None).map_err(|e| {
            error!("Error on grabbing channel uploads for {}", self.name);
            error!("returning empty vec");
        });
        if let Ok(jsn) = jsn {
            let jsn: Value = serde_json::from_str(&jsn).unwrap();
            let vids = jsn["items"].as_array().unwrap();
            let vids: Vec<Video> = iter_collect!(vids, get_vid);
            vids
        } else {
            Vec::new()
        }
    }

    pub fn update_videos(&self) -> bool {
        info!("updating videos for {}", self.name);
        let new = self.get_videos(false);
        let old = self.load_videos();
        let mut new: Vec<Video> = new.into_iter().take_while(|x| x.id != old[0].id).collect();
        if new.len() != 0 {
            info!("found {} new videos for {}", new.len(), self.name);
            new.extend(old);
            self.save_vidoes(&new);
            return true;
        }
        false
    }
    pub fn save_vidoes(&self, vids: &Vec<Video>) {
        json_file!(
            write vids,
            &static_format!("{}/{}/{}_vids.json", CACHE_PATH, self.id, self.id)
        );
    }
    pub fn load_videos(&self) -> Vec<Video> {
        let json = fs::read_to_string(&static_format!(
            "{}/{}/{}_vids.json",
            CACHE_PATH,
            self.id,
            self.id
        ))
        .expect("");
        let json: Vec<Video> = serde_json::from_str(&json).expect("");
        json
    }
}
