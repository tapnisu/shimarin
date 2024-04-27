use crate::exports::*;
use poise::serenity_prelude::{self as serenity};

/// Display info about user from GitHub
#[poise::command(slash_command, prefix_command)]
pub async fn ghuser(
    ctx: Context<'_>,
    #[description = "User to search for"] query: String,
) -> Result<(), Error> {
    let page = octocrab::instance()
        .search()
        .users(&query.trim())
        .per_page(1)
        .send()
        .await?;

    if page.items.is_empty() {
        let reply = {
            let embed = serenity::CreateEmbed::default().title("User not found!");

            poise::CreateReply::default().embed(embed).ephemeral(true)
        };

        ctx.send(reply).await?;

        return Ok(());
    }

    let u = &page.items[0];

    let embed = serenity::CreateEmbed::default()
        .title(u.login.to_owned())
        .url(u.url.to_owned())
        .thumbnail(u.avatar_url.to_owned())
        .fields(vec![("ID".to_owned(), u.id.to_string(), true)]);

    let components = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new_link(u.url.to_owned()).label("Open in browser"),
    ])];

    let reply = poise::CreateReply::default()
        .embed(embed)
        .components(components);

    ctx.send(reply).await?;
    Ok(())
}

/// Display info about repository from GitHub
#[poise::command(slash_command, prefix_command)]
pub async fn ghrepo(
    ctx: Context<'_>,
    #[description = "Repository to search for"] query: String,
) -> Result<(), Error> {
    let page = octocrab::instance()
        .search()
        .repositories(&query.trim())
        .per_page(1)
        .send()
        .await?;

    if page.items.is_empty() {
        let reply = {
            let embed = serenity::CreateEmbed::default().title("Repository not found!");

            poise::CreateReply::default().embed(embed).ephemeral(true)
        };

        ctx.send(reply).await?;

        return Ok(());
    }

    let r = &page.items[0];

    let mut embed = serenity::CreateEmbed::default()
        .title(r.full_name.to_owned().unwrap_or_else(|| r.name.to_owned()))
        .url(r.url.to_owned())
        .fields(vec![("ID", r.id.to_string(), true)]);

    if let Some(desc) = r.description.to_owned() {
        embed = embed.description(desc);
    }

    if let Some(html_url) = r.html_url.to_owned() {
        embed = embed.url(html_url);
    }

    if let Some(watchers_count) = r.watchers_count {
        embed = embed.field("Watchers count", watchers_count.to_string(), true);
    }

    if let Some(forks_count) = r.forks_count {
        embed = embed.field("Forks count", forks_count.to_string(), true);
    }

    if let Some(stargazers_count) = r.stargazers_count {
        embed = embed.field("Stargazers count", stargazers_count.to_string(), true);
    }

    if let Some(fork) = r.fork {
        if fork {
            embed = embed.field("Fork", "Yes", true);
        }
    }

    if let Some(default_branch) = r.default_branch.to_owned() {
        embed = embed.field("Default branch", default_branch, true);
    }

    if let Some(clone_url) = r.clone_url.to_owned() {
        embed = embed.field("Clone url", format!("`{clone_url}`"), false);
    }

    let components = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new_link(r.url.to_owned()).label("Open in browser"),
    ])];

    let reply = poise::CreateReply::default()
        .embed(embed)
        .components(components);

    ctx.send(reply).await?;
    Ok(())
}
