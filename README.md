# RSS-Bot-Telegram
Telegram Bot for RSS feeds.     
This simple bot is made for send update notifications on Telegram from differents RSS feeds.     

# HOW TO USE IT
[Install Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)     

Install dependancies :     
`$_> sudo apt install build-essential`     
`$_> sudo apt install librust-openssl-dev`     

Download project sources :     
`$_> git clone https://github.com/`     

Build project in release mode :     
`$_> cd ./nom_du_dossier_projet`     
`$_> cargo build --release`     

Start bot with token and userID and redirect output to log file :      
`$_> ./target/release/infoscyberbot BOT_TOKEN USER_ID > ./bot.log &`     


&nbsp;


# HOW IT WORKS
This bot take token (get when you create the bot with @BotFather) and your userID (get with @userinfobot) on parameters to start.

When bot is started, it can be managed with the private chat open on his creation. This bot will be able to send RSS update on Telegram channel as admin.      
When it join the channel, it start check and send RSS update (listed on feed.txt) when it get "/start" command.     
To optimize the process this bot check all RSS feeds every 10min and send message on channel every 30secs, to not flood channel.     

Private chat with the bot is use to manage it. The "/start" command list all command it can do.     
Warning all commands only works on the private chat with the bot, commands from channel will be ignored, except "/start" command.     
On the private chat you can add, remove or list RSS feeds, to update feed.txt file.     