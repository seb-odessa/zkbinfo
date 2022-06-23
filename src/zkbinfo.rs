use actix_cors::Cors;
use actix_rt;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use anyhow::anyhow;
use env_logger;
use log::{error, info};
use tokio::time::Duration;

use std::env;

use lib::api;
use lib::database;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("ZKBINFO_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("ZKBINFO_PORT")
        .unwrap_or_default()
        .parse::<u16>()
        .unwrap_or(8080);

    let url = "killmail.db";
    info!("The Database path: {url}");
    let pool = database::create_pool(&url)?;
    info!("Connection to the {url} complete.");
    let cleanup = pool.clone();
    let state = api::AppState::new(pool);
    let context = web::Data::new(state);

    actix_rt::spawn(async move {
        let mut interval = actix_rt::time::interval(Duration::from_secs(60 * 60 * 48));
        loop {
            interval.tick().await;
            if let Ok(conn) = cleanup.get() {
                if let Err(what) = database::cleanup(&conn) {
                    error!("{what}");
                } else {
                    info!("Cleanup performed");
                }
            }
        }
    });

    info!("Launching server at {host}:{port}");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive(), //TODO Rwconfigure due to unsafe
            )
            .app_data(context.clone())
            .service(
                web::scope("/api")
                    .route("/statistic", web::get().to(api::statistic))
                    .route("/killmail/ids/{date}/", web::get().to(api::saved_ids))
                    .route(
                        "/character/{id}/lost/{ship}/",
                        web::get().to(api::character::lost_ship),
                    )
                    .route(
                        "/corporation/{id}/lost/{ship}/",
                        web::get().to(api::character::lost_ship),
                    )
                    .route(
                        "/alliance/{id}/lost/{ship}/",
                        web::get().to(api::character::lost_ship),
                    )
                    .route(
                        "/character/activity/{id}/",
                        web::get().to(api::character::activity),
                    )
                    .route(
                        "/corporation/activity/{id}/",
                        web::get().to(api::corporation::activity),
                    )
                    .route(
                        "/alliance/activity/{id}/",
                        web::get().to(api::alliance::activity),
                    )
                    .route(
                        "/character/activity/hourly/{id}/",
                        web::get().to(api::character::activity_hourly),
                    )
                    .route(
                        "/corporation/activity/hourly/{id}/",
                        web::get().to(api::corporation::activity_hourly),
                    )
                    .route(
                        "/alliance/activity/hourly/{id}/",
                        web::get().to(api::alliance::activity_hourly),
                    )
                    .route(
                        "/character/friends/char/{id}/",
                        web::get().to(api::character::friends_char),
                    )
                    .route(
                        "/character/enemies/char/{id}/",
                        web::get().to(api::character::enemies_char),
                    )
                    .route(
                        "/character/friends/corp/{id}/",
                        web::get().to(api::character::friends_corp),
                    )
                    .route(
                        "/character/enemies/corp/{id}/",
                        web::get().to(api::character::enemies_corp),
                    )
                    .route(
                        "/character/friends/alli/{id}/",
                        web::get().to(api::character::friends_alli),
                    )
                    .route(
                        "/character/enemies/alli/{id}/",
                        web::get().to(api::character::enemies_alli),
                    )
                    .route(
                        "/corporation/friends/char/{id}/",
                        web::get().to(api::corporation::friends_char),
                    )
                    .route(
                        "/corporation/enemies/char/{id}/",
                        web::get().to(api::corporation::enemies_char),
                    )
                    .route(
                        "/corporation/friends/corp/{id}/",
                        web::get().to(api::corporation::friends_corp),
                    )
                    .route(
                        "/corporation/enemies/corp/{id}/",
                        web::get().to(api::corporation::enemies_corp),
                    )
                    .route(
                        "/corporation/friends/alli/{id}/",
                        web::get().to(api::corporation::friends_alli),
                    )
                    .route(
                        "/corporation/enemies/alli/{id}/",
                        web::get().to(api::corporation::enemies_alli),
                    )
                    .route(
                        "/alliance/friends/char/{id}/",
                        web::get().to(api::alliance::friends_char),
                    )
                    .route(
                        "/alliance/enemies/char/{id}/",
                        web::get().to(api::alliance::enemies_char),
                    )
                    .route(
                        "/alliance/friends/corp/{id}/",
                        web::get().to(api::alliance::friends_corp),
                    )
                    .route(
                        "/alliance/enemies/corp/{id}/",
                        web::get().to(api::alliance::enemies_corp),
                    )
                    .route(
                        "/alliance/friends/alli/{id}/",
                        web::get().to(api::alliance::friends_alli),
                    )
                    .route(
                        "/alliance/enemies/alli/{id}/",
                        web::get().to(api::alliance::enemies_alli),
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
