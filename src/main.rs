mod config;
mod discord;
mod events;

use actix_web::{web, App, HttpServer};

#[tokio::main]
async fn main() {
    let config = config::read_config();
    let discord = web::Data::from(discord::start_discord(&config).await);

    println!("Starting HTTP server");

    let config = web::Data::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .app_data(discord.clone())
            .service(events::on_death)
            .service(events::on_join)
    })
    .bind("127.0.0.1:42069")
    .unwrap()
    .run()
    .await
    .unwrap()
}
