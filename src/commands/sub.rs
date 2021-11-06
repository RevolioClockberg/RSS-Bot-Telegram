// Import mod.rs.
use super::*;

pub async fn sub(user_id: tbot::types::chat::Id, cmd: Arc<Command<Text>>) -> Result<(), tbot::errors::MethodCall> {
    let target = &mut MsgTarget::new(user_id, cmd.message_id); 
    let text = &cmd.text.value;
    let feed_url;

    if !text.is_empty() && check_url(text).await {      // If there is a url with the /sub command and URL is valid. 
        feed_url = text.clone();
    } 
    else {
        let msg = tr!("sub_use");
        message(&cmd.bot, target, parameters::Text::with_plain(&msg)).await?;
        return Ok(());
    }
    
    message(&cmd.bot,target,parameters::Text::with_html(tr!("processing"))).await?;

    let mut file = OpenOptions::new().read(true).append(true).open("/path/to/database/feeds.txt").unwrap();     // Open file in read/write mode. 
    let reader = BufReader::new(&file);

    for line in reader.lines() {
        if line.unwrap() == feed_url {          // If the URL is already in file.
            let msg = tr!("sub_success");
            message(&cmd.bot, target, parameters::Text::with_plain(&msg)).await?;
            return Ok(())
        }
    }

    write!(file, "{}\n", feed_url).unwrap();    // Write the URL at the end of the file.  
    let msg = tr!("sub_success");
    message(&cmd.bot, target, parameters::Text::with_plain(&msg)).await?;
    Ok(())
}