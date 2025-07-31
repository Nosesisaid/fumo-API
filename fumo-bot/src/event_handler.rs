use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
            println!(
                "Hi there from the event handler, I'm {}",
                data_about_bot.user.name
            )
        }
        serenity::FullEvent::Message { new_message } => {
            message_create::handler(ctx, framework, data, new_message).await?
        }
        _ => {}
    }

    Ok(())
}
