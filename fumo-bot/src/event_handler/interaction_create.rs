use poise::serenity_prelude::{self as serenity, ComponentInteractionDataKind, InteractionType};

use crate::{util::InteractionCustomID, Data, Error};


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
        

        let custom_id: InteractionCustomID = InteractionCustomID::new(&interaction.data.custom_id);

        let conn = data.db.get()?;

        interaction.defer(ctx).await?;
        match custom_id.action {
            crate::util::InteractionAction::Accept => {
                
            }
            crate::util::InteractionAction::Delete => {}
            _ => {}

        };
        
        

        Ok(())
    }