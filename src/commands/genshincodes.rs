use crate::exports::*;
use poise::serenity_prelude::{self as serenity};
use serde::Deserialize;

const GENSHIN_CODES: &str =
    "https://raw.githubusercontent.com/ataraxyaffliction/gipn-json/main/gipn.json";
const ACTIVATE_GIFT_URL: &str = "https://genshin.hoyoverse.com/en/gift";

#[derive(Debug, Deserialize)]
pub struct Codes {
    #[serde(rename = "CODES")]
    pub codes: Vec<Code>,
}

#[derive(Debug, Deserialize)]
pub struct Code {
    pub reward: String,
    pub date: String,
    pub code: String,
    pub is_expired: bool,
    pub region: u32,
    pub reward_array: Vec<Reward>,
}

#[derive(Debug, Deserialize)]
pub struct Reward {
    pub image_path: String,
    pub name: String,
    pub count: String,
    pub rarity: String,
}

/// Get valid codes for Genshin Impact
#[poise::command(slash_command, prefix_command)]
pub async fn genshincodes(ctx: Context<'_>) -> Result<(), Error> {
    let codes = reqwest::get(GENSHIN_CODES)
        .await?
        .json::<Codes>()
        .await?
        .codes;

    let reply = {
        let embed = serenity::CreateEmbed::default()
            .title("Codes for Genshin Impact")
            .description("You can activate them in game, and get rewards!")
            .url(ACTIVATE_GIFT_URL)
            .fields(
                codes
                    .iter()
                    .filter(|code| !code.is_expired)
                    .map(|code| (code.code.to_owned(), code.reward.to_owned(), true))
                    .collect::<Vec<(String, String, bool)>>(),
            );

        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new_link(ACTIVATE_GIFT_URL).label("Activate"),
        ])];

        poise::CreateReply::default()
            .embed(embed)
            .components(components)
    };

    ctx.send(reply).await?;

    Ok(())
}
