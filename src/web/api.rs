use crate::util_macro::static_format;
use std::error::Error;

use crate::config_reader::*;
use serde_json::Value;

pub struct YTApi {}

impl YTApi {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_self_subscriptions(&self, results: u8) -> Result<Vec<Value>, Box<dyn Error>> {
        let url =static_format!("https://youtube.googleapis.com/youtube/v3/subscriptions?part=snippet%2CcontentDetails&channelId={}&maxResults={}&key={}&order=unread",YT_SELF_CHANNEL_NAME,results,YT_API_KEY);
        let resp = reqwest::blocking::get(url)?.text()?;
        let a: Value = serde_json::from_str(&resp)?;
        let num_pages = a["pageInfo"]["totalResults"].as_u64().unwrap()
            / a["pageInfo"]["resultsPerPage"].as_u64().unwrap();
        let mut ret: Vec<Value> = vec![a];
        for i in 0..num_pages {
            let pg_token = &ret[i as usize]["nextPageToken"].as_str().unwrap();
            let url = static_format!("https://youtube.googleapis.com/youtube/v3/subscriptions?part=snippet%2CcontentDetails&channelId={}&maxResults={}&key={}&pageToken={}&order=unread",YT_SELF_CHANNEL_NAME,results,YT_API_KEY,pg_token);
            let resp = reqwest::blocking::get(url)?.text()?;
            ret.push(serde_json::from_str(&resp)?);
        }
        Ok(ret)
    }
    pub fn get_channel_uploads(
        &self,
        channel_id: String,
        page: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
        if page.is_none() {
            let uploads_playlist = self.get_channel_uploads_id(channel_id)?;
            let url =static_format!("https://youtube.googleapis.com/youtube/v3/playlistItems?part=snippet%2CcontentDetails&maxResults=50&playlistId={}&key={}",uploads_playlist ,YT_API_KEY);
            let resp = reqwest::blocking::get(url)?.text()?;
            Ok(resp)
        } else {
            let uploads_playlist = self.get_channel_uploads_id(channel_id)?;
            let url =static_format!("https://youtube.googleapis.com/youtube/v3/playlistItems?part=snippet%2CcontentDetails&maxResults=50&pageToken={}&playlistId={}&key={}",page.unwrap(),uploads_playlist ,YT_API_KEY);
            let resp = reqwest::blocking::get(url)?.text()?;
        }
    }
    pub(super) fn get_channel_uploads_id(
        &self,
        channel_id: String,
    ) -> Result<String, Box<dyn Error>> {
        let url = static_format!(
            "https://youtube.googleapis.com/youtube/v3/channels?part=contentDetails&id={}&key={}",
            channel_id,
            YT_API_KEY
        );
        let jsn = reqwest::blocking::get(url)?.text()?;
        let jsn: Value = serde_json::from_str(&jsn)?;
        let playlist_id = jsn["items"][0]["contentDetails"]["relatedPlaylists"]["uploads"]
            .as_str()
            .unwrap();
        Ok(playlist_id.to_string())
    }
}
