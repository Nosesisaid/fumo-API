use crate::{Data, Error};
use poise::serenity_prelude as serenity;
mod message_create;


pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Hi there from the event handler, I'm {}", data_about_bot.user.name )
        },
        serenity::FullEvent::Message { new_message } => {
            if new_message.author.id != framework.bot_id {
                return Ok(());
            }

            if new_message.guild_id.is_none() {
                return Ok(());
            }

            if new_message.channel_id != data.administration_channel_id || new_message.guild_id.unwrap() != data.admin_server_id {
                return Ok(());
            }
            println!("Recived a message in the administration channel! {:?}", new_message)
        }
        _ => {}
    }
    
    Ok(())
}