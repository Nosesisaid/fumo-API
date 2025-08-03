
use crate::{Error,Context};


#[poise::command(slash_command, prefix_command)]
pub async fn ping(
    ctx: Context<'_>,
    #[description = "Test option"] test: Option<String>,
) -> Result<(), Error> {
    let Ok(mut _conn) = ctx.data().db.get() else {
        ctx.say("Error getting into the db").await?;
        return Ok(());
    };

    let u = test.unwrap_or("".into());
    let response = format!(
        "Pong! Hi from serenity, I really {}, I'm connected to the db",
        u
    );
    ctx.say(response).await?;
    Ok(())
}