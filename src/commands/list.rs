// Import mod.rs.
use super::*;


pub async fn list(user_id: tbot::types::chat::Id, cmd: Arc<Command<Text>>) -> Result<(), tbot::errors::MethodCall> {
    let target = &mut MsgTarget::new(user_id, cmd.message_id); 
    message(&cmd.bot,target,parameters::Text::with_html(tr!("processing"))).await?;

    let mut msg = String::new();
    let vec_channel = fetch_update().await;
    match vec_channel {
        Ok(vec_channel) => {
            for channel in vec_channel {
                msg.push_str(&format!("- {}\n", channel.title));
            }
        }
        Err(_) => {}
    }

    message(&cmd.bot, target, parameters::Text::with_plain(&msg)).await?;
    Ok(())
}