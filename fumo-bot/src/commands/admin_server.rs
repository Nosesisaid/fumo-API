use std::vec;

use crate::{
    Context, Error, say_ephemeral,
    util::{InvolvableChoice, insert_fumo},
};

use fumo_db::models::NewFumo;
use poise::serenity_prelude::{self as serenity};

#[poise::command(slash_command, subcommands("new"), subcommand_required)]
pub async fn fumo(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, user_cooldown = 5)]
pub async fn new(
    ctx: Context<'_>,
    #[description = "Image of the fumo"] image: serenity::Attachment,
    #[description = "Involved fumos on the image"] involved: Option<InvolvableChoice>,
    #[description = "Transcript of the provided image"] caption: Option<String>,
    #[description = "Special attribution string. Will default to your discord username & id."]
    attribution: Option<String>,
) -> Result<(), Error> {
    let mut invlvd: Vec<Option<String>> = vec![];
    if let Some(inv) = involved {
        invlvd = vec![Some(inv.into())];
    }
    let attribution = attribution.unwrap_or(format!(
        "dsc {}#0000 - {}",
        ctx.author().name,
        ctx.author().id
    ));

    let Some(image_extension) = image.filename.split(".").last() else {
        ctx.say("Invalid file provided").await?;
        return Ok(());
    };
    if !vec!["jpg", "png", "webp", "gif"].contains(&image_extension) {
        ctx.say("File with invalid file extension provided").await?;
        return Ok(());
    };
    let fumo_to_insert = NewFumo {
        img: image.proxy_url.into(), //TODO: ALL The image handling. Checkif if its an image and uploading it to r2
        attribution: attribution,
        caption: caption.unwrap_or_default(),
        public: false,
        submitter: format!("dsc {}-{}", ctx.author().id, ctx.id()),
        involved: invlvd,
    };
    say_ephemeral(ctx, "Loading database and inserting the fumo into it").await?;

    let Ok(mut conn) = ctx.data().db.get() else {
        say_ephemeral(ctx, "Error connection to the database").await?;
        return Ok(());
    };

    let res = insert_fumo(
        &mut conn,
        fumo_to_insert,
        true,
        Some(&ctx),
        Some(&ctx.data()),
    );

    match res {
        Ok(_) => {
            say_ephemeral(ctx, "Fumo succesfully uploaded to the review queue").await?;
            Ok(())
        }
        Err(e) => {
            tracing::warn!{e, "Failed to insert fumo to the review queue"};
            say_ephemeral(ctx, "Error inserting the submission into the review queue.").await?;
            Ok(())
        }
    }
}
