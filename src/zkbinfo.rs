use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use anyhow::anyhow;
use env_logger;
use log::info;

use std::env;

use lib::api;
use lib::database;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let host = env::var("ZKBINFO_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("ZKBINFO_PORT")
        .unwrap_or_default()
        .parse::<u16>()
        .unwrap_or(8080);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let url = "killmail.db";
    info!("The Database path: {url}");
    let pool = database::create_pool(&url)?;
    info!("Connection to the {url} complete.");
    let state = api::AppState::new(pool);
    let context = web::Data::new(state);

    info!("Launching server at {host}:{port}");
    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .service(
                web::scope("/api")
                    .route("/statistic", web::get().to(api::statistic))
                    .route("/killmail/ids/{date}/", web::get().to(api::saved_ids))
                    .route(
                        "/character/activity/{id}/",
                        web::get().to(api::character_activity),
                    )
                    .route(
                        "/character/friends/char/{id}/",
                        web::get().to(api::character_friends_char),
                    )
                    .route(
                        "/character/enemies/char/{id}/",
                        web::get().to(api::character_enemies_char),
                    )
                    .route(
                        "/character/friends/corp/{id}/",
                        web::get().to(api::character_friends_corp),
                    )
                    .route(
                        "/character/enemies/corp/{id}/",
                        web::get().to(api::character_enemies_corp),
                    )
                    .route(
                        "/character/friends/alli/{id}/",
                        web::get().to(api::character_friends_alli),
                    )
                    .route(
                        "/character/enemies/alli/{id}/",
                        web::get().to(api::character_enemies_alli),
                    )
                    .route(
                        "/corporation/friends/char/{id}/",
                        web::get().to(api::corporation_friends_char),
                    )
                    .route(
                        "/corporation/enemies/char/{id}/",
                        web::get().to(api::corporation_enemies_char),
                    )
                    .route(
                        "/corporation/friends/corp/{id}/",
                        web::get().to(api::corporation_friends_corp),
                    )
                    .route(
                        "/corporation/enemies/corp/{id}/",
                        web::get().to(api::corporation_enemies_corp),
                    )
                    .route(
                        "/corporation/friends/alli/{id}/",
                        web::get().to(api::corporation_friends_alli),
                    )
                    .route(
                        "/corporation/enemies/alli/{id}/",
                        web::get().to(api::corporation_enemies_alli),
                    )
                    .route(
                        "/alliance/friends/char/{id}/",
                        web::get().to(api::alliance_friends_char),
                    )
                    .route(
                        "/alliance/enemies/char/{id}/",
                        web::get().to(api::alliance_enemies_char),
                    )
                    .route(
                        "/alliance/friends/corp/{id}/",
                        web::get().to(api::alliance_friends_corp),
                    )
                    .route(
                        "/alliance/enemies/corp/{id}/",
                        web::get().to(api::alliance_enemies_corp),
                    )
                    .route(
                        "/alliance/friends/alli/{id}/",
                        web::get().to(api::alliance_friends_alli),
                    )
                    .route(
                        "/alliance/enemies/alli/{id}/",
                        web::get().to(api::alliance_enemies_alli),
                    ),
            )
            .service(web::scope("/killmail").route("/save", web::post().to(api::save)))
            .wrap(Logger::default())
    })
    .workers(3)
    .bind((host.as_str(), port))?
    .run()
    .await
    .map_err(|e| anyhow!(e))
}
