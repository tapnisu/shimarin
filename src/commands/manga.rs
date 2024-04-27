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
        let embed = serenity::CreateEmbed::default().title("Sorry! Manga not found!");
        let reply = poise::CreateReply::default().embed(embed).ephemeral(true);

        ctx.send(reply).await?;
        return Ok(());
    }

    let manga = &manga_list[0];

    let embed = serenity::CreateEmbed::default()
        .author(serenity::CreateEmbedAuthor::new(&manga.author))
        .title(&manga.name)
        .url(&manga.url)
        .image(&manga.thumbnail)
        .fields(vec![("Last chapter", &manga.last_chapter, true)]);

    let components = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new_link(&manga.url).label("Read manga in browser"),
    ])];

    let reply = poise::CreateReply::default()
        .embed(embed)
        .components(components);

    ctx.send(reply).await?;
    Ok(())
}
