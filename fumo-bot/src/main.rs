use fumo_db::DbPool;
use poise::{serenity_prelude as serenity};

struct Data {
    db: DbPool
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn ping(
    ctx: Context<'_>,
    #[description = "Test option"] test: Option<String>,
) -> Result<(), Error> {
    let Ok(mut conn ) = ctx.data().db.get() else {
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

    _ = dotenvy::dotenv();

    let discord_token =
        std::env::var("FB_DISCORD_API_TOKEN").expect("FB_DISCORD_API_TOKEN not provided");
    let database_url = std::env::var("FB_DATABASE_URL").expect("FB_DATABASE_URL not provided");


    let pool = fumo_db::create_pool(database_url);

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
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
