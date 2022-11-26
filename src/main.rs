use dotenvy::dotenv;
use poise::serenity_prelude::{self as serenity, Activity};
use rand::Rng;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {}

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
        ctx.send(|reply| {
            reply.ephemeral(true);
            reply.embed(|e| e.title("User not found!"))
        })
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
        ctx.send(|reply| {
            reply.ephemeral(true);
            reply.embed(|e| e.title("Repository not found!"))
        })
        .await?;

        return Ok(());
    }

    let r = &page.items[0];

    ctx.send(|reply| {
        reply.embed(|e| {
            if let Some(full_name) = &r.full_name {
                e.title(full_name);
            } else {
                e.title(&r.name);
            }

            if let Some(desc) = &r.description {
                e.description(desc);
            }

            if let Some(html_url) = &r.html_url {
                e.url(&html_url);
            }

            if let Some(watchers_count) = &r.watchers_count {
                e.field("Watchers count", &watchers_count, true);
            }

            if let Some(forks_count) = &r.forks_count {
                e.field("Forks count", &forks_count, true);
            }

            if let Some(stargazers_count) = &r.stargazers_count {
                e.field("Stargazers count", &stargazers_count, true);
            }

            if let Some(clone_url) = &r.clone_url {
                e.field("Clone url", &clone_url, false);
            }

            if let Some(fork) = &r.fork {
                if *fork {
                    e.field("Fork", "true", true);
                }
            }

            if let Some(default_branch) = &r.default_branch {
                e.field("Default branch", default_branch, true);
            }

            e.fields(vec![("ID", &r.id, true)]);

            e
        })
    })
    .await?;
    Ok(())
}

fn gen_password(pass_len: usize) -> String {
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut password = String::with_capacity(pass_len);

    let mut rng = rand::thread_rng();

    for _ in 0..pass_len {
        password.push(charset[rng.gen_range(0..charset.iter().count())])
    }

    password
}

/// Generate password
#[poise::command(slash_command, prefix_command)]
async fn password(
    ctx: Context<'_>,
    #[description = "Length of password"] length: usize,
) -> Result<(), Error> {
    ctx.send(|reply| {
        reply.ephemeral(true);
        reply.embed(|e| {
            e.title("Your password")
                .description("||".to_owned() + &gen_password(length) + &"||".to_owned())
        })
    })
    .await?;

    Ok(())
}

/// Convert HTML to text
#[poise::command(slash_command, prefix_command)]
async fn fromhtml(
    ctx: Context<'_>,
    #[description = "HTML to convert to text"] text: String,
    #[description = "Width of formatted text"] width: Option<usize>,
) -> Result<(), Error> {
    ctx.send(|reply| {
        reply.embed(|e| {
            e.title("Your text").description(html2text::from_read(
                text.as_bytes(),
                if let Some(w) = width { w } else { 20 },
            ))
        })
    })
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![avatar(), password(), user(), ghuser(), ghrepo(), fromhtml()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_activity(Activity::playing("Reading book")).await;
                println!("{} is connected!", _ready.user.tag());
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
