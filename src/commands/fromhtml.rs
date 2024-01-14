use crate::exports::*;
use poise::serenity_prelude::{self as serenity};

/// Convert HTML to text
#[poise::command(slash_command, prefix_command)]
pub async fn fromhtml(
    ctx: Context<'_>,
    #[description = "HTML to convert to text"] text: String,
    #[description = "Width of formatted text"] width: Option<usize>,
) -> Result<(), Error> {
    let reply = {
        let embed = serenity::CreateEmbed::default()
            .title("Your text")
            .description(html2text::from_read(text.as_bytes(), width.unwrap_or(20)));

        poise::CreateReply::default().embed(embed)
    };

    ctx.send(reply).await?;

    Ok(())
}
