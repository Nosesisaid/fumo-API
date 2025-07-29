use fumo_db::{DbPool};
use poise::serenity_prelude::{self as serenity, ChannelId, GuildId};
mod admin_server;


struct Data {
    db: DbPool
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn ping(
    ctx: Context<'_>,
    #[description = "Test option"] test: Option<String>,
) -> Result<(), Error> {
    let Ok(mut _conn ) = ctx.data().db.get() else {
        ctx.say("Error getting into the db").await?;
        return Ok(());
    };

    let u = test.unwrap_or("".into());
    let response = format!("Pong! Hi from serenity, I really {}, I'm connected to the db", u);
    ctx.say(response).await?;
    Ok(())
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    _ = dbg!(dotenvy::dotenv());

    let discord_token =
        std::env::var("FB_DISCORD_API_TOKEN").expect("FB_DISCORD_API_TOKEN not provided");
    let database_url = std::env::var("FB_DATABASE_URL").expect("FB_DATABASE_URL not provided");

    let admin_server_id: GuildId = std::env::var("FB_ADMIN_SERVER_ID").expect("FB_ADMIN_SERVER_ID not provided").parse().expect("Error parsing FB_ADMIN_SERVER_ID into a server id");
    let administraction_channel: ChannelId = std::env::var("FB_ADMINISTRATION_CHANNEL_ID").expect("FB_ADMINISTRATION_CHANNEL_ID not provided").parse().expect("Error parsing FB_ADMINISTRATION_CHANNEL_ID into a channel id");


    let global_commands = vec![ping()];
    let admin_server_commands = vec![admin_server::fumo(),admin_server::new()];

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(),admin_server::fumo(),admin_server::new()],
            pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                let pool = fumo_db::create_pool(database_url);
                println!("Connected to the db pool!");
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &global_commands).await?;
                println!("Succesfully registered {:?} as global commands", &global_commands.iter().map(|c|&c.name).collect::<Vec<&String>>());
                
                poise::builtins::register_in_guild(ctx, &admin_server_commands, admin_server_id).await?;
                println!("Succesfully registered {:?} as commands for {}", &admin_server_commands.iter().map(|c|&c.name).collect::<Vec<&String>>(), &admin_server_id);

                Ok(Data {
                    db: pool
                })
            })
        })
        
        .build();

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
