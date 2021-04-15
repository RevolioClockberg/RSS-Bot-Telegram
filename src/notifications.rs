use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use chrono::DateTime;
use rss::Channel;
use tbot::Bot;

use crate::commands::{fetch_update, push_update};


pub fn start(tbot: Bot, chat_id: tbot::types::chat::Id) {
    let bot = tbot.clone();
    let mut hash_date = HashMap::new();
    tokio::spawn(async move {
        loop {
            let vec_channels: Vec<Channel> = fetch_update().await;                  // Get last update of all feeds.

            for channel in vec_channels {
                let title = channel.title.clone();
                let last = &channel.items().first().unwrap().pub_date().unwrap();    
                let last_pub_online = DateTime::parse_from_rfc2822(last).unwrap();

                if !hash_date.contains_key(&title) || hash_date.get(&title).unwrap() != &last_pub_online {
                    push_update(&bot, channel, chat_id).await.unwrap();
                    hash_date.insert(title, last_pub_online);
                    thread::sleep(Duration::from_secs(30));
                } 
            }
            thread::sleep(Duration::from_secs(600));                                // Check feeds update every 10min.
        }
    });
}