use std::fmt;
use fumo_db::{models::{Fumo, NewFumo}, operations::{add_fumo, PgConnection, QueryResult}};
use poise::serenity_prelude::{ChannelId, Context, CreateEmbed, CreateMessage, UserId};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};
use poise::serenity_prelude as serenity;
use crate::{Data, Error};

//IMPORTANT to keep synced with fumo_db::INVOLVABLE. Haven't found a good way to automate this.
#[derive(Debug, poise::ChoiceParameter, EnumIter, EnumString, IntoStaticStr, Display)]
pub enum InvolvableChoice {
    Cirno,
    Reimu,
    Remilia,
}
impl From<InvolvableChoice> for String {
    fn from(i: InvolvableChoice) -> Self {
        return i.to_string();
    }
}
// impl fmt::Display for InvolvableChoice {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }
// Really, if anyone knows a better way to do this, please tell me. I feel very dumb doing this.


pub fn insert_fumo(conn: &mut PgConnection, to_insert: NewFumo, dispatch: bool, ctx: Option<&serenity::Context>, data: Option<&Data>) ->  Result<Fumo, Error> {

    if dispatch {
        let ctx = ctx.ok_or("If you are dispatching the insertion you must pass on the Context")?;
        let data = data.ok_or("If you are dispatching the insertion you must pass on the data")?;

        let administration_channel = data.administration_channel_id;

        administration_channel.send_message(ctx, CreateMessage::new());

    }

    match add_fumo(conn, to_insert) {
        Ok(f) => Ok(f),
        Err(_)=> Err("Error inserting the fumo".into())
    }
}


fn build_embed_from_newfumo(new: &NewFumo) -> CreateEmbed {

    let (submisstter_id, submission_id) = extract_submitter(new.submitter.as_ref().unwrap()); 

    CreateEmbed::new().title(format!("Submission `#{}`",new.img))
}

fn extract_submitter(submitter_string: String)-> (UserId, u64){


}