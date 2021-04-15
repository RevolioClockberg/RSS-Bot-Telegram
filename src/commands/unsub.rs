// Import mod.rs.
use super::*;


pub async fn unsub(user_id: tbot::types::chat::Id, cmd: Arc<Command<Text>>) -> Result<(), tbot::errors::MethodCall> {
    let target = &mut MsgTarget::new(user_id, cmd.message_id); 
    let text = &cmd.text.value;
    let feed_url;

    if !text.is_empty() {
        feed_url = text.clone();
    } 
    else {
        let msg = tr!("sub_use");
        message(&cmd.bot, target, parameters::Text::with_plain(&msg)).await?;
        return Ok(());
    }

    message(&cmd.bot,target,parameters::Text::with_html(tr!("processing"))).await?;

    let mut vec_new_file = Vec::new();      // To keep all URL for the new file. 
    let mut found = false;
    let mut file = OpenOptions::new().read(true).open("./database/feeds.txt").unwrap();  // Open file in read-only mode. 
    let reader = BufReader::new(&file);

    for line_read in reader.lines() {
        match line_read {
            Ok(s) => {
                if s != feed_url {          // If URL of file is not the specified URL. 
                    vec_new_file.push(s);   // Keep it.
                } else {
                    found = true;           // When the "IF" is not true mean URL was found. 
                }
            },
            Err(_) => println!("Error"),
        }
    }

    if found {      // If URL was found, re-write a file without the specified URL. 
        file = OpenOptions::new().write(true).truncate(true).open("./database/feeds.txt").unwrap();
        for line_write in vec_new_file {
            write!(file, "{}\n", line_write).unwrap();
        }
        let msg = tr!("unsub_success");
        message(&cmd.bot, target, parameters::Text::with_plain(&msg)).await?;
    }
    else {          // If URL not found, re-write a file is useless. 
        let msg = tr!("unsub_fail");
        message(&cmd.bot, target, parameters::Text::with_plain(&msg)).await?;
    }
    Ok(())
}