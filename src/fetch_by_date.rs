// use actix_web::middleware::Logger;
// use actix_web::{web, App, HttpServer};
// use anyhow::anyhow;
use chrono::NaiveDate;
use env_logger;
use log::info;

use std::collections::HashMap;
use std::env;

// use lib::api;
// use lib::database;
// use lib::killmail;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let host = env::var("ZKBINFO_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("ZKBINFO_PORT")
        .unwrap_or_default()
        .parse::<u16>()
        .unwrap_or(8080);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let args = env::args().collect::<Vec<String>>();
    if args.len() != 1 {
        let zkbinfo_api = format!("http://{host}:{port}/killmail/save");
        info!("zkbinfo API url: {zkbinfo_api}");

        if let Ok(date) = NaiveDate::parse_from_str(args[1].as_str(), "%Y-%m-%d") {
            let zkb_api = format!(
                "https://zkillboard.com/api/history/{}.json",
                date.format("%Y%m%d").to_string()
            );

            info!("zkillboard.com API: {zkb_api}");

            let map = reqwest::get(&zkb_api)
                .await?
                .json::<HashMap<i32, String>>()
                .await?;
            info!("Received {} killmails from zkillboard.com", map.len());

            let saved: Vec<u32> = reqwest::get(&zkbinfo_api)
                .await?
                .json()
                .await?;
            info!("Received {} killmails from zkbinfo", saved.len());

            return Ok(());
        }
    }
    Ok(usage(&args[0]))
}

fn usage(app: &String) -> () {
    println!("Usage:\n\t{app} <YYYY-MM-DD>");
}
