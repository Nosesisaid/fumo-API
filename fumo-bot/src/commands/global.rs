
use fumo_db::models::APIFumo;
use poise::{serenity_prelude::CreateEmbed, CreateReply};

use crate::{Error,Context};



#[poise::command(slash_command, subcommands("random"), subcommand_required)]
pub async fn fumo(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}


#[poise::command(slash_command)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {

    let mut conn = ctx.data().db.get()?;

    let random_fumo = fumo_db::operations::get_random(&mut conn, None, false)?;

    let embed = create_embed_for_apifumo(&random_fumo);

    ctx.reply_builder(CreateReply::default().embed(embed));
    Ok(())
}

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


fn create_embed_for_apifumo(f: &APIFumo) -> CreateEmbed{
    CreateEmbed::new()
        .image(&f.img)
        .title(format!("Fumo #{}",&f.id ))
        .description(format!("{}",&f.involved.iter().map(|a|a.clone().unwrap()).collect::<Vec<String>>().join("")))
}