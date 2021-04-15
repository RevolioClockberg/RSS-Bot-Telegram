extern crate rss;
extern crate chrono;
extern crate reqwest;
extern crate tbot;

use std::env;
use tbot::Bot;

// Include the tr! macro and localizations
include!(concat!(env!("OUT_DIR"), "/ctl10n_macros.rs"));

mod commands;
mod notifications;

// Macro to send a parameter to function. 
macro_rules! handle {
    ($env: expr, $f: expr) => {{
        let env = $env.clone();
        let f = $f;
        move |cmd| {
            let future = f(env.clone(), cmd);
            async {
                if let Err(e) = future.await {
                    println!("Error: {}", e);   // Error log. 
                }
            }
        }
    }};
}

#[tokio::main]
async fn main() {
    if env::args().count() != 3 {       // Check if user start bot with token and userID. 
        println!("\nUsage : ./{} BOT_TOKEN USER_ID\n", env::args().next().unwrap());
        std::process::exit(101);
    }

    let bot = Bot::new(env::args().nth(1).unwrap());                                                    // Create bot with token. 
    let user_id = tbot::types::chat::Id::from(env::args().nth(2).unwrap().parse::<i64>().unwrap());     // Set the UserID. 
    let chat_id = tbot::types::chat::Id::from(env::args().nth(3).unwrap().parse::<i64>().unwrap());     // Set ChannelID. 
    
    notifications::start(bot.clone(), chat_id);             // Send automatically update on channel. 
    
    let me = bot.get_me().call().await.unwrap();            // Get bot infos.
    let bot_name = me.user.username.clone().unwrap();       // Get bot name. 

    let mut event_loop = bot.event_loop();                  // Event loop to manage update message witth Telegram. 
    event_loop.username(bot_name);                          // Bot can get command with his name. 
    
    // Associate functions with commands. 
    event_loop.command("start", handle!(user_id, commands::start));
    event_loop.command_if("sub", commands::check_command, handle!(user_id, commands::sub));
    event_loop.command_if("unsub", commands::check_command, handle!(user_id, commands::unsub));
    event_loop.command_if("list", commands::check_command, handle!(user_id, commands::list));

    event_loop.polling().start().await.unwrap();            // Start the loop event.  
}
