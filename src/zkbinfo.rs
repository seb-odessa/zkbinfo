use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use anyhow::anyhow;
use env_logger;
use log::info;

use std::env;

use lib::api;
use lib::database;
use lib::killmail;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let host = env::var("ZKBINFO_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("ZKBINFO_PORT").unwrap_or_default().parse::<u16>().unwrap_or(8080);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let url = "killmail.db";
    info!("The Database path: {url}");
    let connection = database::create_connection(&url)?;
    info!("Connection to the {url} complete.");
    let state = api::AppState::new(connection);
    let context = web::Data::new(state);

    info!("Launching server at {host}:{port}");
    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .service(
                web::scope("/api")
                    .route("/stat", web::get().to(api::statistic))
                    .route("/killmail/saved/{date}/", web::get().to(api::saved_ids))
                    .route("/character/report/{id}/", web::get().to(api::character_report)),
            )
            .service(web::scope("/killmail").route("/save", web::post().to(killmail::save)))
            .wrap(Logger::default())
    })
    .workers(3)
    .bind((host.as_str(), port))?
    .run()
    .await
    .map_err(|e| anyhow!(e))
}
