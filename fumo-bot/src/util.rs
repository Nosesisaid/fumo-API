use std::{str::FromStr, vec};

use crate::{Data, Error};
use fumo_db::{
    models::{Fumo, NewFumo},
    operations::{PgConnection, add_fumo},
};
use poise::serenity_prelude::{CacheHttp, CreateActionRow, CreateButton, CreateEmbed, CreateMessage, InteractionType, Request, Route, UserId};
use poise::serenity_prelude::{CreateEmbedAuthor, Timestamp};
use reqwest::Url;
use strum::{Display, EnumIter, EnumString, IntoStaticStr};
use tracing_subscriber::fmt::format;

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


    if to_insert.involved.iter().any(|i| i.as_deref() == Some("none")){
        to_insert.involved = vec![];
        
    }

    let to_insert_for_embed = to_insert.clone();

    let new_fumo = match add_fumo(conn, to_insert) {
        Ok(f) => f,
        Err(e) =>{ return Err(e.into())},
    };

    if dispatch {
        let ctx = ctx.ok_or("If you are dispatching the insertion you must pass on the Context")?;
        let data = data.ok_or("If you are dispatching the insertion you must pass on the data")?;
        let administration_channel = data.administration_channel_id;
        let embed = build_embed_from_newfumo(&to_insert_for_embed);
        let (submitter_id, submission_id) = extract_submitter(&to_insert_for_embed.submitter);

        tokio::task::block_in_place(||  {

            tokio::runtime::Handle::current().block_on(async {

                let accept_button_id = InteractionCustomID{
                    action: InteractionAction::Accept,
                    submission_id: submission_id,
                    submitter_id: submitter_id.into()
                };


                let delete_button_id = InteractionCustomID{
                    action: InteractionAction::Delete,
                    submission_id: submission_id,
                    submitter_id: submitter_id.into()
                };

                let msg = CreateMessage::new()
                .content(new_fumo.id.to_string())
                .add_embed(embed)
                .components(
                    vec![
                        CreateActionRow::Buttons(vec![
                            CreateButton::new(accept_button_id).label("Accept").style(poise::serenity_prelude::ButtonStyle::Success),
                            CreateButton::new(delete_button_id).label("Delete").emoji('🚮').style(poise::serenity_prelude::ButtonStyle::Danger)
                            ])
                        ]);
            let m = administration_channel.send_message(ctx,msg ).await;
            
            match m {
                Err(e) => tracing::warn!("Error dispatching insert_fumo to the administration channel {}",e),
                _ =>{}
            }
            
            })
            
        });
    }

    Ok(new_fumo)


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


pub trait ToShortStr {
    fn to_short_str(&self) -> String;
}

impl ToShortStr for InteractionType {
    fn to_short_str(&self) -> String {
        match self {
            InteractionType::Autocomplete => "autc",
            InteractionType::Component => "cpnt",
            InteractionType::Modal => "modl",
            _ => "unkn"
        }.into()
    }
}

#[derive(EnumString, Display)]
pub enum InteractionAction {
    Accept,
    Submit,
    Delete,
    Unknown
}



pub struct InteractionCustomID {
//    interaction_kind: InteractionType,
    pub submission_id: u64,
    pub submitter_id: u64,
    pub action: InteractionAction
}

impl InteractionCustomID {
    pub fn new(id: impl Into<String>) -> Result<Self, Error> {

        let id: String = id.into();
        let id = dbg![id.split("-").into_iter().collect::<Vec<&str>>()];

        let submission_id: u64 = id[0].parse()?;
        let submitter_id: u64 = id[1].parse()?;
        let action: InteractionAction = InteractionCustomID::interaction_action_from_short_str(id[2]).unwrap_or(InteractionAction::Unknown);
        Ok(InteractionCustomID { submission_id, submitter_id, action })
    }

    pub fn interaction_action_from_short_str(action: &str) -> Result<InteractionAction, strum::ParseError>{
        InteractionAction::from_str(action)
    }
}

impl From<InteractionCustomID> for String {
    fn from(value: InteractionCustomID) -> Self {
        dbg![format!(
        "{}-{}-{}",
        value.submission_id.to_string(),
        value.submitter_id.to_string(),
        value.action
        )]
    }
}




 pub async fn upload_to_cdn(identificator: &i64, proxy_img_url: impl Into<String>, data: &Data)->Result<String,Error> {

    let proxy_img_url: String = proxy_img_url.into();
    let mut parsed_img_url = Url::parse(&proxy_img_url)?;
    
    parsed_img_url.query_pairs_mut().append_pair("format", "png");
    parsed_img_url.query_pairs_mut().append_pair("quality", "lossless");

    let extension = ".png";
    let key = dbg![format!("{identificator}{extension}")];

    let put_url = format!("{}/{}",data.uploader_worker_base_url,key);
    let client = data.reqwest_client.clone();

    let img_req = client.get(parsed_img_url)
    .send().await?.error_for_status()?;

    let buff = img_req.bytes().await?;


    let res = client
    .put(put_url)
    .header("X-Custom-Auth-Key", &data.worker_auth_key_secret)
    .body(buff)
    .send()
    .await?;

    res.error_for_status()?;

    Ok(format!("{}/{}",data.r2_base_url,key))
}