use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use chrono::DateTime;
use tbot::Bot;

use crate::commands::{fetch_update, push_update};


pub fn start(tbot: Bot, chat_id: tbot::types::chat::Id) {
    let bot = tbot.clone();
    let mut hash_date = HashMap::new();
    tokio::spawn(async move {
        loop {                
            if let Ok(vec_channels) = fetch_update().await {                // Get last update of all feeds.
                for channel in vec_channels {                               // Parse channels vector.
                    let title = channel.title.clone();
                    let last_build = DateTime::parse_from_rfc2822(&channel.items().first().unwrap().pub_date().unwrap()).unwrap();

                    if !hash_date.contains_key(&title) || hash_date.get(&title).unwrap() != &last_build {    // Check if article date is more recent than the last message sent.
                        if let Ok(_) = push_update(&bot, &channel, chat_id.clone()).await{
                            hash_date.insert(title, last_build);
                            thread::sleep(Duration::from_secs(30));          // Wait 30sec between two sent message.
                        }   
                    } 
                }
                thread::sleep(Duration::from_secs(600));                     // Check feeds update every 10min.
            }
        }
    });
}