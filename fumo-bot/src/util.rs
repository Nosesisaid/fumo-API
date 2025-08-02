use crate::{Data, Error};
use fumo_db::{
    models::{Fumo, NewFumo},
    operations::{PgConnection, add_fumo},
};
use poise::serenity_prelude::{CacheHttp, CreateEmbed, CreateMessage, UserId};
use poise::serenity_prelude::{CreateEmbedAuthor, Timestamp};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

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

pub fn insert_fumo(
    conn: &mut PgConnection,
    mut to_insert: NewFumo,
    dispatch: bool,
    ctx: Option<impl CacheHttp>,
    data: Option<&Data>,
) -> Result<Fumo, Error> {
    if dispatch {
        let ctx = ctx.ok_or("If you are dispatching the insertion you must pass on the Context")?;
        let data = data.ok_or("If you are dispatching the insertion you must pass on the data")?;
        let administration_channel = data.administration_channel_id;
        let embed = build_embed_from_newfumo(&to_insert);
        tokio::task::block_in_place(||  {

            tokio::runtime::Handle::current().block_on(async {
            let m = administration_channel.send_message(ctx, CreateMessage::new().add_embed(embed)).await;
            
            match m {
                Err(e) => println!("Error dispatching insert_fumo to the administration channel {}",e),
                _ =>{}
            }
            
            })
            
        });
    }


    if to_insert.involved.iter().any(|i| i.as_deref() == Some("none")){
        to_insert.involved = vec![];
        
    }

    match add_fumo(conn, to_insert) {
        Ok(f) => Ok(f),
        Err(_) => Err("Error inserting the fumo".into()),
    }
}

fn build_embed_from_newfumo(new: &NewFumo) -> CreateEmbed {
    let (submitter_id, submission_id) = extract_submitter(&new.submitter);

    let involved_in_string = new
        .involved
            .iter()
                .filter_map(|o| o.as_ref())
                .cloned()
                .collect::<Vec<String>>()
                .join(", ");

    CreateEmbed::new()
        .title(format!("Submission `#{}`", submission_id))
        .image(&new.img)
        .author(CreateEmbedAuthor::new(format!(
            "By <@{}> [{}]",
            &submitter_id, &submitter_id
        )))
        .field("Involved", format!("`{}`", involved_in_string), false)
        .field("Caption", &new.caption, true)
        .field("Attribution", &new.attribution, false)
        .field("Proxy image url", format!("`{}`", &new.img), false)
        .timestamp(Timestamp::now())
}

fn extract_submitter(submitter_string: &String) -> (UserId, u64) {
    let submitter_string = submitter_string
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .to_string();
    let mut submitter_string = submitter_string.split("-");

    let submitter_id: u64 = submitter_string
        .next()
        .unwrap()
        .to_string()
        .parse()
        .unwrap();

    let submission_id = submitter_string.last().unwrap().parse().unwrap();

    return (UserId::new(submitter_id), submission_id);
}
