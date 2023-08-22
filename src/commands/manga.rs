use crate::exports::*;
use poise::serenity_prelude::{self as serenity};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MangaSearchItem {
    pub id: String,
    pub name: String,
    #[serde(rename = "lastChapter")]
    pub last_chapter: String,
    pub thumbnail: String,
    pub author: String,
    pub url: String,
}

/// Get data about manga
#[poise::command(slash_command, prefix_command)]
pub async fn manga(
    ctx: Context<'_>,
    #[description = "Query for search"] query: String,
) -> Result<(), Error> {
    let manga_list: Vec<MangaSearchItem> =
        reqwest::get(format!("https://manga.deno.dev/api/search?q=\"{}\"", query))
            .await?
            .json()
            .await?;

    if manga_list.is_empty() {
        ctx.send(|reply| {
            reply.ephemeral(true);
            reply.embed(|e| e.title("Sorry! Manga not found!"))
        })
        .await?;

        return Ok(());
    }

    let manga = &manga_list[0];

    ctx.send(|reply| {
        reply.embed(|e| {
            e.title(&manga.name)
                .url(&manga.url)
                .image(&manga.thumbnail)
                .fields(vec![("Last chapter", &manga.last_chapter, true)])
        });

        reply.components(|c| {
            c.add_action_row(
                serenity::CreateActionRow::default()
                    .add_button(
                        serenity::CreateButton::default()
                            .label("Read manga in browser")
                            .url(&manga.url)
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
