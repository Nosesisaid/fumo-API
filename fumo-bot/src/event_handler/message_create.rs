use std::{
    time::{Duration, SystemTime, UNIX_EPOCH},
    vec,
};

use crate::{
    Data, Error,
    util::{InvolvableChoice, insert_fumo},
};
use fumo_db::models::NewFumo;
use poise::serenity_prelude::{
    self as serenity, ComponentInteractionDataKind, CreateButton, CreateEmbed, CreateEmbedAuthor,
    CreateEmbedFooter, CreateInputText, CreateInteractionResponseFollowup, CreateMessage,
    CreateQuickModal, CreateSelectMenu, CreateSelectMenuOption, EditMessage,
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

        //Wait one second bcuz rust is so blazingly fast responding before bugs out the discord client
        tokio::time::sleep(Duration::from_secs(1)).await;
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

        let involvable_choice_valid_iter = InvolvableChoice::iter()
            .map(|f| CreateSelectMenuOption::new(f.to_string(), f.to_string()));
        let mut select_menu_options = vec![
            CreateSelectMenuOption::new("None", "none")
                .emoji('🛑')
                .description("N/A"),
        ];
        select_menu_options.extend(involvable_choice_valid_iter);
        let select_menu_len = select_menu_options.len().try_into().unwrap();

        dbg!(&select_menu_options);
        let menu_id = format!("submit-men-{}", new_message.id);
        let button_id = format!("submit-{}", new_message.id);

        let mut submission_embed = CreateEmbed::new()
                            .title(format!("Submission `#{}`", &new_message.id))
                            .author(CreateEmbedAuthor::new(format!(
                                "Submitter 'dsc {}-{}'",
                                &new_message.author.id,
                                &new_message.id
                            )))
                            .description("Click on the submit button to fill all the neccesary data for your submission.")
                            .footer(CreateEmbedFooter::new("Do not delete the submission message. Your submission will be discarded automatically"))
                            .color(serenity::Colour::PURPLE);

        let mut embed_subm_message = thread
            .send_message(
                ctx,
                CreateMessage::new()
                    .add_embed(submission_embed.clone())
                    .button(
                        CreateButton::new(&button_id)
                            .label("Submit")
                            .style(serenity::ButtonStyle::Primary)
                            .disabled(true),
                    )
                    .select_menu(
                        CreateSelectMenu::new(
                            &menu_id,
                            serenity::CreateSelectMenuKind::String {
                                options: select_menu_options,
                            },
                        )
                        .max_values(select_menu_len),
                    ),
            )
            .await?;

        let mut successfully_submitted = false;

        let mut insertable = NewFumo {
            attribution: "".into(),
            caption: "".into(),
            img: attachment.proxy_url.clone(),
            involved: vec![],
            public: false,
            submitter: format!("dsc {}-{}", new_message.author.id, new_message.id),
        };

        let menu_collector = embed_subm_message
            .await_component_interaction(&ctx.shard)
            .timeout(Duration::from_secs(INTERACTION_COLLECTOR_TIMEOUT))
            .filter(move |mci| mci.data.custom_id == menu_id.clone());

        let mut selected_involved: Option<Vec<String>> = None;
        if let Some(mci) = menu_collector.await {
            println!("Menu interaction in submission");

            _ = mci
                .create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
                .await;

            if let ComponentInteractionDataKind::StringSelect { values } = &mci.data.kind {
                submission_embed =
                    submission_embed
                        .clone()
                        .field("Involved", values.join(" "), false);

                selected_involved = Some(values.clone());
                _ = embed_subm_message
                    .edit(
                        ctx,
                        EditMessage::new().embed(submission_embed.clone()).button(
                            CreateButton::new(&button_id)
                                .label("Submit")
                                .disabled(false)
                                .style(serenity::ButtonStyle::Primary),
                        ),
                    )
                    .await;
                insertable.involved = values.clone().iter().map(|i| Some(i.clone())).collect()
            }
        }

        dbg!(&selected_involved);

        let button_collector = embed_subm_message
            .await_component_interaction(&ctx.shard)
            .timeout(Duration::from_secs(INTERACTION_COLLECTOR_TIMEOUT))
            .filter(move |mci| mci.data.custom_id == button_id);
        if let Some(mci) = button_collector.next().await {
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
            let provided_attribution = if provided_attribution.is_empty() {
                format!("Discord {} [{}]", mci.user.name, mci.user.id)
            } else {
                provided_attribution.clone()
            };
            dbg!(&provided_caption, &provided_attribution);

            insertable.attribution = provided_attribution.clone();
            insertable.caption = provided_caption.clone();

            submission_embed = submission_embed
                .clone()
                .field("Caption", provided_caption, false)
                .field("Attribution", provided_attribution, false);

            embed_subm_message
                .edit(
                    ctx,
                    EditMessage::new().embed(submission_embed).button(
                        CreateButton::new("none")
                            .style(serenity::ButtonStyle::Success)
                            .disabled(true)
                            .label("Sent"),
                    ),
                )
                .await?;

            res.interaction
                .create_followup(
                    ctx,
                    CreateInteractionResponseFollowup::new().content("Trying to submit fumo..."),
                )
                .await?;

            let mut conn = data.db.get()?;

            let r = insert_fumo(&mut conn, insertable, true, Some(ctx), Some(data));
            'set_success: {
                match r {
                    Ok(_) => {
                        res.interaction.create_followup(ctx, CreateInteractionResponseFollowup::new().content("Submission succesfully sent to review! Thanks for contributing to the Fumo-API.")).await?;
                    }
                    Err(e) => {
                        println!("Error inserting the fumo{}",e);
                        res.interaction.create_followup(ctx, CreateInteractionResponseFollowup::new().content("**X** Error trynig to send the submission to review. Thanks for (trying to) contribute to the Fumo-API.")).await?;

                        break 'set_success;
                    }
                };

                successfully_submitted = true;
            }
        }

        let message_edition: EditMessage = EditMessage::new().button(
            CreateButton::new("none")
                .style(serenity::ButtonStyle::Danger)
                .disabled(true)
                .label("Timed out"),
        );
        if !successfully_submitted {
            embed_subm_message.edit(ctx, message_edition).await?;
        }
    };
    Ok(())
}

fn get_unix_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
