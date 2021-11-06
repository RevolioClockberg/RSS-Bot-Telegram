use std::error::Error;
use std::sync::Arc;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use rss::Channel;
use chrono::DateTime;
use reqwest::{self, Url};
use dissolve::strip_html_tags;
use tbot::{
    contexts::{Command, Text},
    types::parameters,
    types::chat::Kind,
    Bot,
};
// Import sub-modules.
mod start;
mod sub;
mod unsub;
mod list;

// Make these functions usable in the main.rs. 
pub use start::start;
pub use sub::sub;
pub use unsub::unsub;
pub use list::list;



#[derive(Debug, Copy, Clone)]
struct MsgTarget {
    chat_id: tbot::types::chat::Id,
    message_id: tbot::types::message::Id,
    first_time: bool,
}

impl MsgTarget {
    fn new(chat_id: tbot::types::chat::Id, message_id: tbot::types::message::Id) -> Self {
        MsgTarget {
            chat_id,
            message_id,
            first_time: true,
        }
    }
    fn update(&mut self, message_id: tbot::types::message::Id) {
        self.message_id = message_id;
        self.first_time = false;
    }
}


// Send message on Telegram.
async fn message(
    bot: &Bot,
    target: &mut MsgTarget,
    message: parameters::Text<'_>,
) -> Result<(), tbot::errors::MethodCall> {
    let msg = if target.first_time {
        bot.send_message(target.chat_id, message)
            .is_web_page_preview_disabled(true)
            .call()
            .await?
    } else {
        bot.edit_message_text(target.chat_id, target.message_id, message)
            .is_web_page_preview_disabled(true)
            .call()
            .await?
    };
    target.update(msg.id);
    Ok(())
}


// Check if a command is sent from a Telegram Channel. 
pub async fn check_command(cmd: Arc<Command<Text>>) -> bool {
    match cmd.chat.kind {
        Kind::Private { .. } => return true,
        _ => return false,
    }
}


// Check if an url can join RSS feeds list. 
async fn check_url(feed_url: &str) -> bool {
    let url = Url::parse(feed_url).unwrap(); // Ignore errors.

    match reqwest::get(url).await {
        Ok(s) => {
            let content = s.bytes().await;
            match content{
                Ok(content) => {
                    let channel = Channel::read_from(&content[..]);
                    match channel {
                        Ok(channel) => { let item = channel.items.first().unwrap(); 
                                        match item.pub_date() {
                                            Some(_) => return true,
                                            None => return false,
                                        }}
                        Err(_) => return false
                    }
                }
                Err(_) => return false
            }
        },
        Err(_) => return false,
    };
}


pub async fn fetch_update() -> Result<Vec<Channel>, ()> {
    let file = OpenOptions::new().read(true).open("/path/to/database/feeds.txt").unwrap();      // Open file in read-only mode. 
    let reader = BufReader::new(&file);

    let mut vec_channels = Vec::new();
    let mut thread_handles = vec![];

    for line in reader.lines() {      // Read database file.
        let url = line.unwrap();
        let channel = get_feed(&url).await;

        match channel {
            Err(_) => {},
            Ok(channel) => {
                thread_handles.push(channel);
            }
        }
    }
    
    for handle in thread_handles {
        vec_channels.push(handle);      // Create a vector of channels
    }
    Ok(vec_channels)
}


async fn get_feed(url: &String) -> Result<Channel, Box<dyn Error>> {      // Get new from RSS feed URL.
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}


pub async fn push_update(bot: &Bot, channel: &Channel, chat_id: tbot::types::chat::Id,) -> Result<(), tbot::errors::MethodCall> {
    if let Some(item) = channel.items().first() {      // Get the last post of feed.
        let date = {
            match item.pub_date() {      // Get publication date.
                Some(date) => DateTime::parse_from_rfc2822(date).unwrap().format("%Y/%m/%d à %H:%M").to_string(),
                None => tr!("no_date").to_string(),
            }
        };

        let link: String = {
            match item.guid() {      // Get link to article.
                Some(guid) => {
                    if guid.is_permalink() {
                        guid.value().to_string()
                    } else {
                        item.link().unwrap().to_string()
                    }
                },
                None => item.link().unwrap().to_string(),
            }
        };

        let mut desc: String = {
            match item.description() {      // Get description.
                Some(desc) => {
                    desc.to_string()
                }
                None => tr!("no_description").to_string(),
            }
        };

        if let Some(title) = item.title() {      // Get title.
            title.to_string();

            desc = strip_html_tags(&desc).join(" ");                                // Get out all HTML tag. 
            let offset = desc.find('.').unwrap_or(desc.len());                      // Keep only the first sentence of
            desc = desc.drain(..offset).collect();                                  // description from the last post feed. 
            
            let msg = format!("{}\n{}\n\n{}.\n{}", title, link, desc, date);        // Build the message. 
            bot.send_message(chat_id, parameters::Text::with_plain(&msg)).is_web_page_preview_disabled(true).call().await?;
        }
    }
    Ok(())
}