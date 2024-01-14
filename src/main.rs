mod commands;
mod exports;

use commands::*;
use dotenvy::dotenv;
use exports::Data;
use poise::serenity_prelude::{self as serenity, ActivityData};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                avatar(),
                fromhtml(),
                genshincodes(),
                // ghrepo(),
                ghuser(),
                manga(),
                password(),
                user(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("sr!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_activity(Some(ActivityData::playing("Reading book")));

                println!("{} is connected!", ready.user.tag());

                Ok(Data {})
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
