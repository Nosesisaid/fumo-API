use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use tracing_subscriber::fmt::format;
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
            if new_message.author.id == framework.bot_id {
                return Ok(());
            }

            if new_message.guild_id.is_none() {
                return Ok(());
            }

            if new_message.guild_id.unwrap() != data.admin_server_id {
                return Ok(());
            }

            if new_message.channel_id == data.submissions_channel_id {

            let attachment = new_message.attachments.first();
            if attachment.is_none(){
                const SECONDS_TO_WAIT_BEFORE_DELETING: u64 = 15;
                let timestamp_to_delete = get_unix_epoch()+SECONDS_TO_WAIT_BEFORE_DELETING;
                let warning_msg = new_message.reply(ctx, format!("You must provide a attachment in your submission. Deleting invalid submission <t:{}:R>", timestamp_to_delete)).await?;

                let http = ctx.http.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_secs(SECONDS_TO_WAIT_BEFORE_DELETING)).await;
                    http.delete_message(new_message.channel_id, warning_msg.id, Some("Automated deletion"))
                });
                return Ok(())
            }

            let thread = new_message.channel_id.create_thread_from_message(ctx, new_message.id, serenity::CreateThread::new(format!("Fumo submission #{}",new_message.id))).await?;

            println!("{:?}", new_message.attachments);
            if new_message.attachments.len() > 1 {
                thread.say(ctx, "More than one attachment detected in the submission. Only the first one will be parsed").await?;
            }


            thread.say(ctx, format!("**Submission #{}**\n\n Click on the buttons", new_message.id)).await?;
            thread.say(ctx, "Do not delete this message. If you do so your submission ill be discarded").await?;
            println!("Recived a message in the administration channel! {:?}", new_message)
            };
        }
        _ => {}
    }
    
    Ok(())
}

fn get_unix_epoch() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}