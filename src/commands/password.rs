use crate::exports::*;
use poise::serenity_prelude::{self as serenity};
use rand::Rng;

const DEFAULT_PASSWORD_CHARSET: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

// Generates password string
pub fn generate_password(charset: &str, pass_len: usize) -> String {
    let charset: Vec<char> = charset.chars().collect();
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
    #[description = "Charset to use when generating password (a-z,A-Z,0-9 by default)"]
    charset: Option<String>,
) -> Result<(), Error> {
    let password = generate_password(
        &charset.unwrap_or(DEFAULT_PASSWORD_CHARSET.to_owned()),
        length,
    );

    let embed = serenity::CreateEmbed::default()
        .title("Your password")
        .description(format!("||{}||", password));

    let reply = poise::CreateReply::default().embed(embed).ephemeral(true);

    ctx.send(reply).await?;
    Ok(())
}
