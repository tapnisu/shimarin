use crate::exports::*;

/// Convert HTML to text
#[poise::command(slash_command, prefix_command)]
pub async fn fromhtml(
    ctx: Context<'_>,
    #[description = "HTML to convert to text"] text: String,
    #[description = "Width of formatted text"] width: Option<usize>,
) -> Result<(), Error> {
    ctx.send(|reply| {
        reply.embed(|e| {
            e.title("Your text").description(html2text::from_read(
                text.as_bytes(),
                if let Some(w) = width { w } else { 20 },
            ))
        })
    })
    .await?;

    Ok(())
}
