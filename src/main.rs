use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Displays info about user
#[poise::command(slash_command, prefix_command)]
async fn user(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    ctx.send(|reply| {
        reply.embed(|e| {
            e.title(u.tag());

            if let Some(avatar_url) = u.clone().avatar_url() {
                e.thumbnail(avatar_url);
            }

            e.fields(vec![("ID".to_string(), u.id.to_string(), true)]);

            e
        })
    })
    .await?;
    Ok(())
}

/// Displays info about user
#[poise::command(slash_command, prefix_command)]
async fn avatar(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    ctx.send(|reply| {
        reply.embed(|e| {
            e.title(u.tag());

            if let Some(avatar_url) = u.clone().avatar_url() {
                e.image(avatar_url);
            }

            e
        })
    })
    .await?;
    Ok(())
}

/// Display info about user from GitHub
#[poise::command(slash_command, prefix_command)]
async fn ghuser(
    ctx: Context<'_>,
    #[description = "User to search for"] query: String,
) -> Result<(), Error> {
    let page = octocrab::instance()
        .search()
        .users(&query.trim())
        .per_page(1)
        .send()
        .await?;

    if &page.items.len() == &0usize {
        ctx.send(|reply| reply.embed(|e| e.title("User not found!")))
            .await?;

        return Ok(());
    }

    let u = &page.items[0];

    ctx.send(|reply| {
        reply.embed(|e| {
            e.title(&u.login);
            e.url(&u.url);
            e.thumbnail(&u.avatar_url);

            e.fields(vec![("ID", &u.id, true)]);

            e
        })
    })
    .await?;
    Ok(())
}

/// Display info about repository from GitHub
#[poise::command(slash_command, prefix_command)]
async fn ghrepo(
    ctx: Context<'_>,
    #[description = "Repository to search for"] query: String,
) -> Result<(), Error> {
    let page = octocrab::instance()
        .search()
        .repositories(&query.trim())
        .per_page(1)
        .send()
        .await?;

    if &page.items.len() == &0usize {
        ctx.send(|reply| reply.embed(|e| e.title("Repository not found!")))
            .await?;

        return Ok(());
    }

    let r = &page.items[0];

    ctx.send(|reply| {
        reply.embed(|e| {
            e.title(&r.name);
            if let Some(desc) = &r.description {
                e.description(desc);
            }

            if let Some(html_url) = &r.html_url {
                e.url(&html_url);
            }

            e.fields(vec![("ID", &r.id, true)]);

            if let Some(clone_url) = &r.clone_url {
                e.field("Clone url", &clone_url, true);
            }

            if let Some(stargazers_count) = &r.stargazers_count {
                e.field("Stargazers count", &stargazers_count, true);
            }

            if let Some(forks_count) = &r.forks_count {
                e.field("Forks count", &forks_count, true);
            }

            if let Some(fork) = &r.fork {
                if *fork {
                    e.field("Fork", "true", true);
                }
            }

            if let Some(default_branch) = &r.default_branch {
                e.field("Default branch", default_branch, true);
            }

            e
        })
    })
    .await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), avatar(), register(), user(), ghuser(), ghrepo()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}
