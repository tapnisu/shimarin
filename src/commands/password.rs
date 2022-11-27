use crate::exports::*;

/// Generate password
#[poise::command(slash_command, prefix_command)]
pub async fn password(
    ctx: Context<'_>,
    #[description = "Length of password"] length: usize,
) -> Result<(), Error> {
    ctx.send(|reply| {
        reply.ephemeral(true);
        reply.embed(|e| {
            e.title("Your password")
                .description("||".to_owned() + &gen_password(length) + &"||".to_owned())
        })
    })
    .await?;

    Ok(())
}
