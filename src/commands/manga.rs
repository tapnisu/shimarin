use crate::exports::*;
use serde::{Deserialize, Serialize};

use poise::serenity_prelude::{self as serenity};

#[derive(Debug, Serialize, Deserialize)]
struct MangaSearchItem {
    id: String,
    name: String,
    #[serde(rename = "lastChapter")]
    last_chapter: String,
    thumbnail: String,
    author: String,
    url: String,
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

    if manga_list.len() == 0 {
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
            e.title(&manga.name);
            e.url(&manga.url);
            e.image(&manga.thumbnail);

            e.fields(vec![("Last chapter", &manga.last_chapter, true)]);

            e
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
