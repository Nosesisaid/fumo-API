use poise::{serenity_prelude::{self as serenity, ChannelId, GuildId}};
use crate::{Context,Error};

//IMPORTANT to keep synced with fumo_db::INVOLVABLE. Haven't found a good way to automate this.
#[derive(Debug, poise::ChoiceParameter)]
enum InvolvableChoice {
    Cirno,
    Reimu,
    Remilia,
}



#[poise::command(slash_command, subcommands("new"), subcommand_required)]
pub async fn fumo(_: Context<'_>) -> Result<(), Error>{
    Ok(())
}


#[poise::command(slash_command)]
pub async fn new(
    ctx: Context<'_>,
    #[description = "Image of the fumo"] image: serenity::Attachment,
    #[description = "Involved fumos on the image"]involved: Option<InvolvableChoice>
) -> Result<(), Error>{
    ctx.reply("hiii").await?;
    Ok(())

}