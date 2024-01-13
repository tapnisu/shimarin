use crate::exports::*;
use poise::serenity_prelude::{self as serenity};

/// Get user's avatar
#[poise::command(slash_command, prefix_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let reply = {
        let embed = serenity::CreateEmbed::default()
            .title(u.tag())
            .image(u.face());

        poise::CreateReply::default().embed(embed)
    };

    ctx.send(reply).await?;

    Ok(())
}
