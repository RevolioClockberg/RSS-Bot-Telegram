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
                let last_pub = DateTime::parse_from_rfc2822(&channel.items().first().unwrap().pub_date().unwrap()).unwrap();

                if !hash_date.contains_key(&title) || hash_date.get(&title).unwrap() != &last_pub {
                    push_update(&bot, &channel, chat_id.clone()).await.unwrap();
                    hash_date.insert(title, last_pub);
                    thread::sleep(Duration::from_secs(30));
                } 
            }
            thread::sleep(Duration::from_secs(600));                                // Check feeds update every 10min.
        }
    });
}