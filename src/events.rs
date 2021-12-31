use actix_web::{post, web, HttpResponse};
use futures::TryFutureExt;
use serde::Deserialize;
use serenity::{builder::CreateEmbed, CacheAndHttp};

use crate::config::Config;

#[derive(Debug, Deserialize)]
struct MemberDetails {
    username: String,
    position: String,
}

#[post("/events/on_death")]
async fn on_death(
    config: web::Data<Config>,
    discord: web::Data<CacheAndHttp>,
    member_details: web::Json<MemberDetails>,
) -> HttpResponse {
    let position = member_details.position.clone();

    report(config, discord, member_details, |e| {
        e.title("💀 Death")
            .description(format!("Position: ``{}``", position))
    })
    .await;

    HttpResponse::Ok().finish()
}

#[post("/events/on_join")]
async fn on_join(
    config: web::Data<Config>,
    discord: web::Data<CacheAndHttp>,
    member_details: web::Json<MemberDetails>,
) -> HttpResponse {
    let position = member_details.position.clone();

    report(config, discord, member_details, |e| {
        e.title("😀 Join")
            .description(format!("Position: ``{}``", position))
    })
    .await;

    HttpResponse::Ok().finish()
}

async fn report<F>(
    config: web::Data<Config>,
    discord: web::Data<CacheAndHttp>,
    member_details: web::Json<MemberDetails>,
    embed: F,
) where
    F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
{
    println!("{:?}", member_details);

    while let Some(n) = config.members.iter().next() {
        if member_details.username.eq(&n.username) {
            let discord0 = discord.clone();

            match n
                .discord
                .create_dm_channel(&discord.http)
                .and_then(
                    |x| async move { x.send_message(&discord0.http, |m| m.embed(embed)).await },
                )
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    println!("Failed to report to player `{}`: {:?}", n.discord.0, e);
                }
            }

            break;
        }
    }
}
