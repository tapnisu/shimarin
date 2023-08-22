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

    ctx.send(|reply| {
        reply.embed(|e| {
            e.title("Codes for Genshin Impact")
                .description("You can activate them in game, and get rewards!")
                .url(ACTIVATE_GIFT_URL);

            e.fields(
                codes
                    .iter()
                    .filter(|code| !code.is_expired)
                    .map(|code| (code.code.clone(), code.reward.clone(), true))
                    .collect::<Vec<(String, String, bool)>>(),
            )
        });

        reply.components(|c| {
            c.add_action_row(
                serenity::CreateActionRow::default()
                    .add_button(
                        serenity::CreateButton::default()
                            .label("Activate")
                            .url(ACTIVATE_GIFT_URL)
                            .style(serenity::ButtonStyle::Link)
                            .to_owned(),
                    )
                    .to_owned(),
            )
        })
    })
    .await?;

    Ok(())
}
