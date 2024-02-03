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

    let reply = {
        let embed = serenity::CreateEmbed::default()
            .title(u.login.to_owned())
            .url(u.url.to_owned())
            .thumbnail(u.avatar_url.to_owned())
            .fields(vec![("ID".to_owned(), u.id.to_string(), true)]);

        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new_link(u.url.to_owned()).label("Open in browser"),
        ])];

        poise::CreateReply::default()
            .embed(embed)
            .components(components)
    };

    ctx.send(reply).await?;

    Ok(())
}

// /// Display info about repository from GitHub
// #[poise::command(slash_command, prefix_command)]
// pub async fn ghrepo(
//     ctx: Context<'_>,
//     #[description = "Repository to search for"] query: String,
// ) -> Result<(), Error> {
//     let page = octocrab::instance()
//         .search()
//         .repositories(&query.trim())
//         .per_page(1)
//         .send()
//         .await?;

//     if page.items.is_empty() {
//         let reply = {
//             let embed = serenity::CreateEmbed::default().title("Repository not found!");

//             poise::CreateReply::default().embed(embed).ephemeral(true)
//         };

//         ctx.send(reply).await?;

//         return Ok(());
//     }

//     let r = &page.items[0];

//     ctx.send(|reply| {
//         reply.embed(|e| {
//             if let Some(full_name) = &r.full_name {
//                 e.title(full_name);
//             } else {
//                 e.title(&r.name);
//             }

//             if let Some(desc) = &r.description {
//                 e.description(desc);
//             }

//             if let Some(html_url) = &r.html_url {
//                 e.url(html_url);
//             }

//             if let Some(watchers_count) = &r.watchers_count {
//                 e.field("Watchers count", watchers_count, true);
//             }

//             if let Some(forks_count) = &r.forks_count {
//                 e.field("Forks count", forks_count, true);
//             }

//             if let Some(stargazers_count) = &r.stargazers_count {
//                 e.field("Stargazers count", stargazers_count, true);
//             }

//             if let Some(clone_url) = &r.clone_url {
//                 e.field("Clone url", clone_url, false);
//             }

//             if let Some(fork) = &r.fork {
//                 if *fork {
//                     e.field("Fork", "true", true);
//                 }
//             }

//             if let Some(default_branch) = &r.default_branch {
//                 e.field("Default branch", default_branch, true);
//             }

//             e.fields(vec![("ID", &r.id, true)]);

//             e
//         });

//         reply.components(|c| {
//             c.add_action_row(
//                 serenity::CreateActionRow::default()
//                     .add_button(
//                         serenity::CreateButton::default()
//                             .label("Open in browser")
//                             .url(&r.url)
//                             .style(serenity::ButtonStyle::Link)
//                             .to_owned(),
//                     )
//                     .to_owned(),
//             )
//         })
//     })
//     .await?;

//     Ok(())
// }
