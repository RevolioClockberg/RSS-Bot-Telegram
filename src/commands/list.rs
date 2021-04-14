// Import mod.rs.
use super::*;


pub async fn list(user_id: tbot::types::chat::Id, cmd: Arc<Command<Text>>) -> Result<(), tbot::errors::MethodCall> {
    let file = OpenOptions::new().read(true).open("./feeds.txt").unwrap();      // Open file in read-only mode. 
    let reader = BufReader::new(&file);

    let mut msg = String::new();
    for line in reader.lines() {
        let url = Url::parse(line.unwrap().as_str()).unwrap();
        let content = reqwest::get(url).await.unwrap().bytes().await.unwrap();
        let title = Channel::read_from(&content[..]).unwrap().title;            // Get only title of every feeds.  

        msg.push_str(&format!("- {}\n", title));      // Make one message with all title feed. 
    }
    message(&cmd.bot, user_id, parameters::Text::with_plain(&msg)).await?;
    Ok(())
}