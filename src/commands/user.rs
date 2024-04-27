use crate::exports::*;
use poise::serenity_prelude::{self as serenity};

/// Get info about user
#[poise::command(slash_command, prefix_command)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let embed = serenity::CreateEmbed::default()
        .title(u.tag())
        .thumbnail(u.face())
        .fields(vec![("ID".to_string(), u.id.to_string(), true)])
        .image(u.banner_url().unwrap_or("".to_owned()));

    let reply = poise::CreateReply::default().embed(embed);

    ctx.send(reply).await?;
    Ok(())
}
