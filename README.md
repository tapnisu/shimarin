<p align="center">
   <img width=400px src="assets/avatar.png" alt="Bot logo">
   <h1 align="center">RShima</h1>
   <h3 align="center">Multipurpose discord bot</h3>
</p>

<p align="center">
   <a href="#overview">Overview</a>,
   <a href="#invite">Invite</a>,
   <a href="#host">Host</a>
</p>

## Overview

RShima is a discord bot, made using `Rust`. Bot is named after Rin Shima from
Yuru Camp manga.

## Invite

You can invite the bot by
[this link](https://discord.com/api/oauth2/authorize?client_id=1038694628490235904&permissions=1806070768902&scope=bot%20applications.commands).
Type "/" to see commands!

## Host

1. Create bot -
   [Discord developer portal](https://discord.com/developers/applications).

2. Install [rust](https://www.rust-lang.org/).

3. Set up your [.env](.env.sample) (sample in [.env.sample](.env.sample)).

```env
DISCORD_TOKEN="DISCORD_BOT_TOKEN_FROM_<https://discord.com/developers/applications>"
```

4. Start bot - `cargo run -r`

There is a better way, but lol.