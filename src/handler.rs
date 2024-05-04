use poise::serenity_prelude::*;

pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        ctx.set_activity(Some(ActivityData::playing("Reading book")));
    }
}
