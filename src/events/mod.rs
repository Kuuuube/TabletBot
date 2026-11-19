use poise::serenity_prelude as serenity;
use ::serenity::all::prelude::CacheHttp;

use crate::{Data, Error, commands};

pub mod code;
pub mod issues;

pub struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn dispatch(&self, ctx: &serenity::Context, event: &serenity::FullEvent) {
        let _ = event_handler(ctx, event, &ctx.data()).await;
    }
}

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    data: &Data,
) -> Result<(), Error> {
    #[allow(clippy::single_match)]
    match event {
        serenity::FullEvent::Ready { data_about_bot: _, .. } => {
            let commands =  vec![
                commands::register(),
                commands::snippets::snippet(),
                commands::snippets::create_snippet(),
                commands::snippets::remove_snippet(),
                commands::snippets::export_snippet(),
                commands::snippets::list_snippets(),
                commands::snippets::edit_snippet(),
                commands::utils::embed(),
                commands::utils::edit_embed(),
                commands::utils::add_repo(),
                commands::utils::remove_repo(),
                commands::utils::list_repos(),
                commands::udev::generate_udev(),
            ];
            poise::builtins::register_globally(ctx.http(), &commands).await?;
        }
        serenity::FullEvent::Message { new_message , .. } => {
            if !new_message.author.bot() && new_message.guild_id.is_some() {
                issues::message(data, ctx, new_message).await;
                code::message(ctx, new_message).await;
            }
        }
        _ => (),
    }

    Ok(())
}
