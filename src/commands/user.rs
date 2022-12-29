use crate::exports::*;
use poise::serenity_prelude::{self as serenity};

/// Displays info about user
#[poise::command(slash_command, prefix_command)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    ctx.send(|reply| {
        reply.embed(|e| {
            e.title(u.tag());

            e.thumbnail(u.clone().face());

            if let Some(banner_url) = u.clone().banner_url() {
                e.image(banner_url);
            }

            e.fields(vec![("ID".to_string(), u.id.to_string(), true)]);

            e
        })
    })
    .await?;

    Ok(())
}
