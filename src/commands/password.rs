use crate::exports::*;
use rand::Rng;

// Generates password string
pub fn gen_password(pass_len: usize) -> String {
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut password = String::with_capacity(pass_len);

    let mut rng = rand::thread_rng();

    for _ in 0..pass_len {
        password.push(charset[rng.gen_range(0..charset.len())])
    }

    password
}

/// Generate password
#[poise::command(slash_command, prefix_command)]
pub async fn password(
    ctx: Context<'_>,
    #[description = "Length of password"] length: usize,
) -> Result<(), Error> {
    ctx.send(|reply| {
        reply.embed(|e| {
            e.title("Your password")
                .description(format!("||{}||", gen_password(length)))
        });

        reply.ephemeral(true)
    })
    .await?;

    Ok(())
}
