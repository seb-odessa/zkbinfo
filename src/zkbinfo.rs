use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use anyhow::anyhow;
use env_logger;
use log::info;

use lib::api;
use lib::database;
use lib::killmail;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let url = "killmail.db";
    info!("The Database path: {url}");
    let connection = database::create_connection(&url)?;
    info!("Connection to the {url} complete.");
    let state = api::AppState::new(connection);
    let context = web::Data::new(state);

    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .service(
                web::scope("/api")
                    .route("/stat", web::get().to(api::statistic))
                    .route("/killmail/saved/{date}/", web::get().to(api::saved_ids)),
            )
            .service(web::scope("/killmail").route("/save", web::post().to(killmail::save)))
            .wrap(Logger::default())
    })
    .workers(3)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(|e| anyhow!(e))
}
