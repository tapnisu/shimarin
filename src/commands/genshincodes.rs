use crate::exports::*;

/// Get valid codes for Genshin Impact
#[poise::command(slash_command, prefix_command)]
pub async fn genshincodes(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|reply| {
        reply.ephemeral(true);
        reply.embed(|e| e.title("Genshin codes"))
    })
    .await?;

    Ok(())
}
