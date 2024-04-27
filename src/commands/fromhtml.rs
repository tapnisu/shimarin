use crate::exports::*;
use poise::serenity_prelude::{self as serenity};

/// Convert HTML into text
#[poise::command(slash_command, prefix_command)]
pub async fn fromhtml(
    ctx: Context<'_>,
    #[description = "Source HTML"] text: String,
    #[description = "Width of formatted text"] width: Option<usize>,
) -> Result<(), Error> {
    let embed = serenity::CreateEmbed::default()
        .title("Your text")
        .description(html2text::from_read(text.as_bytes(), width.unwrap_or(20)));

    let reply = poise::CreateReply::default().embed(embed);

    ctx.send(reply).await?;
    Ok(())
}
