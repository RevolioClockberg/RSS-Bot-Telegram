use std::time::Duration;
use std::thread;
use chrono::{Local, DateTime};

// Import mod.rs.
use super::*;


pub async fn start(user_id: tbot::types::chat::Id, cmd: Arc<Command<Text>>) -> Result<(), tbot::errors::MethodCall> {
    let chat_id = cmd.chat.id;
    if user_id == chat_id {                                                     // If start command come from admin user. 
        let msg = tr!("start_message");
        message(&cmd.bot, user_id, parameters::Text::with_html(&msg)).await?;
        return Ok(());
    }

    let msg = tr!("channel_start_message");
    message(&cmd.bot, chat_id, parameters::Text::with_plain(&msg)).await?;

    let mut last_message = Local::now();
    loop {
        let vec_channels: Vec<Channel> = fetch_update().await;                  // Get last update of all feeds.

        for channel in vec_channels {
            let last = channel.items().first().unwrap().pub_date().unwrap();    
            let last_pub_online = DateTime::parse_from_rfc2822(last).unwrap();

            // If a less amount of time between now and the last publication of an item is passed compared to the amount of time betwenn now 
            // and the last message send, that's mean a new publication has appeared. So we send it and update the time of last message send. 
            if Local::now().signed_duration_since(last_pub_online) < Local::now().signed_duration_since(last_message) {
                push_update(&cmd, channel).await?;                              // Send feeds update on the channel.
                last_message = Local::now();                                    // Update of last message send.
            }
            thread::sleep(Duration::from_secs(30));                             // Break to not flood the channel.  
        }
        thread::sleep(Duration::from_secs(600));                                // Check feeds update every 10min.
    }
}