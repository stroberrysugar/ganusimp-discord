use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::prelude::Ready,
    CacheAndHttp, Client,
};
use std::sync::Arc;

use crate::config::Config;

pub async fn start_discord(config: &Config) -> Arc<CacheAndHttp> {
    let mut client = Client::builder(&config.token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    let cache_and_http = client.cache_and_http.clone();

    {
        let mut data = client.data.write().await;
        data.insert::<Config>(config.clone());
    }

    tokio::spawn(async move {
        println!("Starting Discord");

        match client.start().await {
            Ok(_) => {}
            Err(e) => println!("Error starting Discord: {:?}", e),
        }

        println!("Stopping Discord");
    });

    cache_and_http
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("Discord ready");
    }
}
