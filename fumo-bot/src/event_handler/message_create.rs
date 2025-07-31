use std::{
    time::{Duration, SystemTime, UNIX_EPOCH},
    vec,
};

use crate::{Data, Error, util::InvolvableChoice};
use poise::serenity_prelude::{
    self as serenity, ComponentInteractionDataKind, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateInputText, CreateMessage, CreateQuickModal, CreateSelectMenu, CreateSelectMenuOption, EditMessage, MessageBuilder
};
use strum::IntoEnumIterator;


const INTERACTION_COLLECTOR_TIMEOUT: u64 = 120;


pub async fn handler(
    ctx: &serenity::Context,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
    new_message: &serenity::Message,
) -> Result<(), Error> {
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
        println!("Message recived in the submissions channel");
        let attachment = new_message.attachments.first();
        if attachment.is_none() {
            const SECONDS_TO_WAIT_BEFORE_DELETING: u64 = 10;
            let timestamp_to_delete = get_unix_epoch() + SECONDS_TO_WAIT_BEFORE_DELETING;

            let warning_msg_id = new_message.reply(ctx, format!("You must provide an attachment in your submission. Deleting invalid submission <t:{}:R>", timestamp_to_delete)).await?.id;
            let http = ctx.http.clone();
            let channel_id = new_message.channel_id;
            let parent_message_id = new_message.id;
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(SECONDS_TO_WAIT_BEFORE_DELETING - 1)).await;
                _ = tokio::join!(
                    http.delete_message(channel_id, warning_msg_id, Some("Automated deletion")),
                    http.delete_message(channel_id, parent_message_id, Some("Automated deletion"))
                );
            });
            return Ok(());
        }
        let attachment = attachment.unwrap();

        let thread = new_message
            .channel_id
            .create_thread_from_message(
                ctx,
                new_message.id,
                serenity::CreateThread::new(format!("Fumo submission #{}", new_message.id)),
            )
            .await?;

        println!("{:?}", new_message.attachments);
        if new_message.attachments.len() > 1 {
            thread.say(ctx, "More than one attachment detected in the submission. Only the first one will be parsed").await?;
        }

        let Some(image_extension) = attachment.filename.split(".").last() else {
            thread.say(ctx, "Invalid file provided").await?;
            return Ok(());
        };
        if !vec!["jpg", "png", "webp", "gif"].contains(&image_extension) {
            thread
                .say(ctx, "File with invalid file extension provided")
                .await?;
            return Ok(());
        };


        let involvable_choice_valid_iter = InvolvableChoice::iter().map(|f| CreateSelectMenuOption::new(f.to_string(),f.to_string()));
        let mut select_menu_options = vec![CreateSelectMenuOption::new("None", "none").emoji('🛑').description("N/A")];
        select_menu_options.extend(involvable_choice_valid_iter);

        let menu_id = format!("submit-men-{}", new_message.id);
        let button_id = format!("submit-{}", new_message.id);
        
        
        let mut submission_embed = CreateEmbed::new()
                            .title(format!("Submission `#{}`", &new_message.id))
                            .author(CreateEmbedAuthor::new(format!(
                                "Submitter 'dsc-{}'",
                                &new_message.author.id
                            )))
                            .description("Click on the submit button to fill all the neccesary data for your submission.")
                            .footer(CreateEmbedFooter::new("Do not delete the submission message. Your submission will be discarded automatically"))
                            .color(serenity::Colour::PURPLE);
        
        let mut embed_subm_message = thread
            .send_message(
                ctx,
                CreateMessage::new()
                    .add_embed(
                        submission_embed,
                    )
                    .button(
                        CreateButton::new(&button_id)
                            .label("Submit")
                            .style(serenity::ButtonStyle::Primary),
                    )
                    .select_menu(
                        CreateSelectMenu::new(&menu_id, serenity::CreateSelectMenuKind::String {
                             options: select_menu_options, 
                            }).max_values(10)
                    )
                    
                    ,
            )
            .await?;

        let mut succesfully_submitted = false;




        let menu_collector = serenity::ComponentInteractionCollector::new(&ctx.shard)
        .timeout(Duration::from_secs(INTERACTION_COLLECTOR_TIMEOUT))
        .filter(move |mci| mci.data.custom_id == menu_id);

        // while let Some(mci) = menu_collector.await {
        //     println!("Menu interaction in submission");

        //     if let ComponentInteractionDataKind::StringSelect { values } = &mci.data.kind {
                
        //     }
        // }

        let button_collector = serenity::ComponentInteractionCollector::new(&ctx.shard)
            .timeout(Duration::from_secs(INTERACTION_COLLECTOR_TIMEOUT))
            .filter(move |mci| mci.data.custom_id == button_id);
        while let Some(mci) = button_collector.await {
            println!("Submission button press detected. Showing modal.");
            let modal = CreateQuickModal::new("Finish your submission")
                .field(
                    CreateInputText::new(serenity::InputTextStyle::Paragraph, "Caption", "")
                        .placeholder("Transcription of the text in the submission. Not required.")
                        .required(false),
                )
                .field(
                    CreateInputText::new(serenity::InputTextStyle::Short, "Attribution", "")
                        .placeholder(format!(
                            "Defaults to \"Discord {} [{}]\".",
                            mci.user.name, mci.user.id
                        ))
                        .required(false),
                )
                .timeout(Duration::from_secs(INTERACTION_COLLECTOR_TIMEOUT));
            let res = mci.quick_modal(ctx, modal).await?;
            let res = res.ok_or("Error getting the modal response")?;

            res.interaction
                .create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
                .await?;

            let inputs = res.inputs;

            let (provided_caption, provided_attribution) = (&inputs[0], &inputs[1]);
            dbg!(provided_caption, provided_attribution);

            succesfully_submitted = true;
            break;
        }

        let mut message_edition: EditMessage = EditMessage::new().button(
            CreateButton::new("none")
                .style(serenity::ButtonStyle::Danger)
                .disabled(true)
                .label("Timed out"),
        );
        if succesfully_submitted {
            message_edition = EditMessage::new().button(
                CreateButton::new("none")
                    .style(serenity::ButtonStyle::Success)
                    .disabled(true)
                    .label("Sent"),
            );
        }

        embed_subm_message.edit(ctx, message_edition).await?;
    };
    Ok(())
}

fn get_unix_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
