// Import mod.rs.
use super::*;


pub async fn start(user_id: tbot::types::chat::Id, cmd: Arc<Command<Text>>) -> Result<(), tbot::errors::MethodCall> {
    let target = &mut MsgTarget::new(user_id, cmd.message_id); 
    let msg = tr!("start_message");
    message(&cmd.bot, target, parameters::Text::with_plain(&msg)).await?;
    Ok(())
}