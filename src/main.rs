mod commands;
mod exports;

use commands::*;
use exports::Data;
use dotenvy::dotenv;
use poise::serenity_prelude::{self as serenity, Activity};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                avatar(),
                fromhtml(),
                genshincodes(),
                ghrepo(),
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
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_activity(Activity::playing("Reading book")).await;

                println!("{} is connected!", ready.user.tag());

                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
