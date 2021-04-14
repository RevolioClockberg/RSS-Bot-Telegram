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


// Send message on Telegram.
async fn message(bot: &Bot, target:tbot::types::chat::Id, message: parameters::Text<'_>) -> Result<(), tbot::errors::MethodCall> {
    bot.send_message(target, message).is_web_page_preview_disabled(true).call().await?;
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
            let content = s.bytes().await.unwrap();
            let channel = Channel::read_from(&content[..]).unwrap();
            let item = channel.items.first().unwrap(); 
            match item.pub_date() {
                Some(_) => return true,
                None => return false,
            }
        },
        Err(_) => return false,
    };
}

// Get updates from feeds.
async fn fetch_update() -> Vec<Channel> {
    let mut vec_channels = Vec::new();
    let file = OpenOptions::new().read(true).open("./feeds.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let url = Url::parse(line.unwrap().as_str()).unwrap();
        let content = reqwest::get(url).await.unwrap().bytes().await.unwrap();
        let channel = Channel::read_from(&content[..]).unwrap();

        vec_channels.push(channel);
    }
    vec_channels
}

// Send feeds update on Telegram. 
async fn push_update(cmd: &Arc<Command<Text>>, channel: Channel) -> Result<(), tbot::errors::MethodCall> {
    let item = channel.items.first().unwrap();          // Get the last post of feed.
    let date = {
        match item.pub_date() {
            Some(date) => DateTime::parse_from_rfc2822(date).unwrap().format("%Y/%m/%d à %H:%M").to_string(),
            None => tr!("no_date").to_string(),
        }
    };
    let title = item.title().unwrap().to_string();
    let link: String = { 
        match item.guid() {
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
        match item.description() {
            Some(desc) => {
                desc.to_string()
            }
            None => tr!("no_description").to_string(),
        }
    };
    desc = strip_html_tags(&desc).join(" ");                                // Get out all HTML tag. 
    let offset = desc.find('.').unwrap_or(desc.len());                      // Keep only the first sentence of
    desc = desc.drain(..offset).collect();                                  // description from the last post feed. 
    
    let msg = format!("{}\n{}\n\n{}.\n{}", title, link, desc, date);        // Build the message. 
    message(&cmd.bot, cmd.chat.id, parameters::Text::with_html(&msg)).await?;
    Ok(())
}