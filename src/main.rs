use poise::serenity_prelude as serenity;
use sqlx;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqliteJournalMode};
use std::env;
use std::str::FromStr;

mod fudge_dice;
mod characters;
mod commands;

struct Data {
    pub db: SqlitePool
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Missing discord token.");
    let intents = serenity::GatewayIntents::non_privileged();
    let db_url = &env::var("DATABASE_URL").expect("Unable to read DATABASE_URL environment variable.");
    let options = SqliteConnectOptions::from_str(db_url).expect("Unable to read database.")
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);
    let pool = SqlitePool::connect_with(options).await.expect("Database connection failed!");
    sqlx::migrate!().run(&pool).await.expect("Unable to run migrations.");
    let data = Data { db: pool.clone() };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions{
            commands: vec![commands::fudge(), commands::xfudge()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
