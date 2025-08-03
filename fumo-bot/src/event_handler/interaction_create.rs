use poise::serenity_prelude::{self as serenity, ChannelId, CreateInteractionResponseFollowup, EditMessage, EditThread, InteractionType};

use crate::{util::{upload_to_cdn, InteractionCustomID}, Data, Error};


pub async fn handler(
    ctx: &serenity::Context,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
    interaction: &serenity::Interaction,
) -> Result<(), Error> {
    match interaction.kind() {
        InteractionType::Component => {component_interaction_handler(ctx, framework, data, interaction.as_message_component().unwrap()).await},
        _ => {Ok(())},
    }
}

async fn component_interaction_handler(
    ctx: &serenity::Context,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
    interaction: &serenity::ComponentInteraction) -> Result<(), Error>{
        

        let custom_id: InteractionCustomID = if let Ok(i) = InteractionCustomID::new(&interaction.data.custom_id){
                i
        } else {
            //Unparsable interaction
            return Ok(())
        };

        let mut conn = data.db.get()?;

        let submsission_message = data.submissions_channel_id.message(ctx, custom_id.submission_id).await?; 

        interaction.defer(ctx).await?;
        match custom_id.action {
            crate::util::InteractionAction::Accept => {
                let submission_public_thread = ChannelId::new(custom_id.submission_id);



                let mut submission_admin_message = interaction.message.clone();

                let entry_id: i64 = interaction.message.content.trim().parse()?;

                
                let new_image_url = upload_to_cdn(&entry_id,&submsission_message.attachments.first().ok_or("Attachment not found on submission message.")?.proxy_url, &data).await?;

                let followup = interaction.create_followup(ctx, CreateInteractionResponseFollowup::new().content("Trying to approve entry...")).await?;
            
                let res = fumo_db::operations::approve_entry(&mut conn, entry_id, Some(new_image_url));

                match res {
                    Ok(f) => {
                                   submission_admin_message.edit(ctx, EditMessage::new().components(vec![])).await?;

                        submission_public_thread.say(ctx, "Accepted :)").await?;
                        dbg!(submission_public_thread.edit_thread(ctx, EditThread::new().archived(true).locked(true)).await?);
                        interaction.edit_followup(ctx, followup.id, CreateInteractionResponseFollowup::new().content("Succesfully accepted, entry is now public")).await?;
                    }
                    Err(e)=>{
                        interaction.edit_followup(ctx, followup.id, CreateInteractionResponseFollowup::new().content("Error updating the db. Not updated")).await?;
          
                        return Err(e.into());

                    }
                }

            }
            crate::util::InteractionAction::Delete => {
                todo!()
            }
            _ => {}

        };
        
        

        Ok(())
    }

