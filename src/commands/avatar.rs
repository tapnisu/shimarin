use crate::exports::*;
use poise::serenity_prelude::{self as serenity};

/// Send user's avatar
#[poise::command(slash_command, prefix_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    ctx.send(|reply| {
        reply.embed(|e| {
            e.title(u.tag());

            if let Some(avatar_url) = u.clone().avatar_url() {
                e.image(avatar_url);
            }

            e
        })
    })
    .await?;
    Ok(())
}
