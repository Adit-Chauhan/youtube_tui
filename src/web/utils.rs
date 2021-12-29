use std::error::Error;
use std::fs;
use std::io::Write;


use crate::config_reader::*;
use crate::util_macro::*;
use itertools::Itertools;

use rayon::prelude::*;
use regex::Regex;
use reqwest::header;

use serde_json::Value;

use crate::web::api::YTApi;
use crate::web::{Video, YTChannel};

pub fn get_recent() -> Result<Vec<Video>, Box<dyn Error>> {
    #[rustfmt::skip]
    let get_vi = |x:&Value| {x
        .as_array()
        .unwrap()
        .iter()
        .map(|vid| -> Video {
            Video::new(
                vid_travel!(vid, "gridVideoRenderer", "videoId"),
                vid_travel!(vid, "gridVideoRenderer", "title", "runs", 0, "text"),
                vid_travel!(vid,"gridVideoRenderer","thumbnail","thumbnails",2,"url"),
                vid_travel!(vid, "gridVideoRenderer", "viewCountText", "simpleText"),
                vid_travel!(vid, "gridVideoRenderer", "publishedTimeText", "simpleText"),
                vid_travel!(vid,"gridVideoRenderer","shortBylineText","runs",0,"text"),
                vid_travel!(vid,"gridVideoRenderer","shortBylineText","runs",0,"navigationEndpoint","browseEndpoint","canonicalBaseUrl"),
            )
        })
        .collect()
};

    let json = filter_json("https://www.youtube.com/feed/subscriptions")?;
    #[rustfmt::skip]
    let days = json_travel!(json,"contents","twoColumnBrowseResultsRenderer","tabs",0,"tabRenderer","content","sectionListRenderer","contents");
    #[rustfmt::skip]
    let today = json_travel!(days,0,"itemSectionRenderer","contents",0,"shelfRenderer","content","gridRenderer","items");
    #[rustfmt::skip]
    let yesterday = json_travel!(days,1,"itemSectionRenderer","contents",0,"shelfRenderer","content","gridRenderer","items");
    #[rustfmt::skip]
    let past = json_travel!(days,2,"itemSectionRenderer","contents",0,"shelfRenderer","content","gridRenderer","items");

    let mut today_vids: Vec<Video> = get_vi(today);
    let yes_vids: Vec<Video> = get_vi(yesterday);
    let past_vids: Vec<Video> = get_vi(past);
    today_vids.extend(yes_vids);
    today_vids.extend(past_vids);
    Ok(today_vids)
}

pub fn get_home() -> Result<Vec<Video>, Box<dyn Error>> {
    let json = filter_json("https://www.youtube.com/")?;

    #[rustfmt::skip]
    let home = json_travel!(json,"contents","twoColumnBrowseResultsRenderer","tabs",0,"tabRenderer","content","richGridRenderer","contents");

    #[rustfmt::skip]
    let home_vids: Vec<Video> = home.as_array().unwrap().iter().map(|vid| -> Video {
        Video::new(
            vid_travel!(vid,"richItemRenderer","content","videoRenderer","videoId"),
            vid_travel!(vid,"richItemRenderer","content","videoRenderer","title","runs",0,"text"),
            vid_travel!(vid,"richItemRenderer","content","videoRenderer","thumbnail","thumbnails", 1,"url"),
            vid_travel!(vid,"richItemRenderer","content","videoRenderer","viewCountText","simpleText"),
            vid_travel!(vid,"richItemRenderer","content","videoRenderer","publishedTimeText","simpleText"),
            vid_travel!(vid,"richItemRenderer","content","videoRenderer","longBylineText","runs",0,"text"),
            vid_travel!(vid,"richItemRenderer","content","videoRenderer","longBylineText","runs",0,"navigationEndpoint","browseEndpoint","canonicalBaseUrl"),
            )
        }).collect::<Vec<Video>>();

    let home_vids = home_vids
        .into_iter()
        .filter(|x| -> bool { x.id != "" })
        .collect();

    Ok(home_vids)
}

fn filter_json(url: &str) -> Result<Value, Box<dyn Error>> {
    let mut request_cookie = header::HeaderMap::new();
    unsafe {
        request_cookie.insert(header::COOKIE, header::HeaderValue::from_static(YT_COOKIES));
    }
    let client = reqwest::blocking::Client::builder()
        .default_headers(request_cookie)
        .build()
        .unwrap();
    let html = client.get(url).send()?.text()?;
    let re = Regex::new(r"var ytInitialData =.* (\{.*\});</script>")?;
    let caps = re.captures(&html).unwrap();
    let text = caps.get(1).map_or("", |m| m.as_str());
    let json: Value = serde_json::from_str(text)?;
    Ok(json)
}

pub fn get_channels() -> Result<Vec<YTChannel>, Box<dyn Error>> {
    let api = YTApi::new();
    let s = api.get_self_subscriptions(50)?;
    let mut v: Vec<YTChannel> = Vec::new();

    for json in s {
        let items = json["items"].as_array().unwrap();
        for item in items {
            #[rustfmt::skip]
            v.push(YTChannel::new(
                vid_travel!(item, "snippet", "title"),
                vid_travel!(item, "snippet", "resourceId", "channelId"),
                vid_travel!(item, "snippet", "description"),
                vid_travel!(item, "thumbnails", "high","url"),
                vid_travel!(item, "contentDetails", "totalItemCount"),
            ));
        }
    }

    Ok(v)
}

pub fn save_channels_initial(channels: &Vec<YTChannel>) {
    println!("saving Channels");
    let chans_list: Vec<String> = channels
        .par_iter()
        .map(|c| -> String {
            c.save_channel();
            static_format!("{}/{}/{}", CACHE_PATH, c.id, c.id)
        })
        .collect();
    json_file!(
        write & chans_list,
        &static_format!("{}/channels.json", CACHE_PATH)
    );
}

pub fn load_channels() -> Vec<YTChannel> {
    let chans_list =
        fs::read_to_string(&static_format!("{}/channels.json", CACHE_PATH)).expect("Opened file");
    let chans_list: Vec<String> = serde_json::from_str(&chans_list).unwrap();
    let channels: Vec<YTChannel> = chans_list
        .into_iter()
        .map(|c| -> YTChannel {
            let op = fs::read_to_string(&format!("{}.json", c)).expect("file");
            let temp: YTChannel = serde_json::from_str(&op).unwrap();
            temp
        })
        .collect();
    channels
}

pub fn save_channel_vids() {
    println!("Saving Videos");
    let chans = load_channels();
    println!("Loaded Channels");
    chans
        .par_iter()
        .map(|c| {
            c.save_vidoes(static_format!("{}/{}/{}", CACHE_PATH, c.id, c.id));
        })
        .collect::<Vec<_>>();
}
pub fn update_channels() {
    println!("updating Channels");

    let chans_list =
        fs::read_to_string(&static_format!("{}/channels.json", CACHE_PATH)).expect("Opened file");
    let chans_list: Vec<String> = serde_json::from_str(&chans_list).unwrap();
    let chans_back = chans_list.clone();
    println!("Total of {} channels", chans_list.len());
    let mut updated: Vec<String> = chans_list
        .into_par_iter()
        .filter(|c| -> bool {
            let old = fs::read_to_string(&format!("{}_vids.json", c)).expect("");
            let old: Vec<Video> = serde_json::from_str(&old).expect("");
            let new = fs::read_to_string(&format!("{}.json", c)).expect("");
            let new: YTChannel = serde_json::from_str(&new).expect("");
            let new = new.get_videos(true);
            if new[0].id != old[0].id {
                json_file!(write & new, &format!("{}_vids.json", c));
                return true;
            }
            false
        })
        .collect();
    println!("New Videos from {} channels", updated.len());
    updated.extend(chans_back);
    let new_list: Vec<String> = updated.into_iter().unique().collect();
    println!("total of {} channels", new_list.len());
    json_file!(
        write & new_list,
        &static_format!("{}/channels.json", CACHE_PATH)
    );
}
