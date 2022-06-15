use chrono::NaiveDate;
use env_logger;
use log::{error, info, warn};
use tokio::time::Duration;

use std::collections::HashMap;
use std::env;

use lib::killmail;

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
        let client = reqwest::Client::new();
        let zkbinfo_save_api = format!("http://{host}:{port}/killmail/save");
        info!("zkbinfo API SAVE url: {zkbinfo_save_api}");

        if let Ok(date) = NaiveDate::parse_from_str(args[1].as_str(), "%Y-%m-%d") {
            let zkbinfo_get_saved_api = format!(
                "http://{host}:{port}/api/killmail/saved/{}/",
                date.format("%Y-%m-%d").to_string()
            );
            info!("zkbinfo API GET_SAVED: {zkbinfo_get_saved_api}");

            let zkb_api = format!(
                "https://zkillboard.com/api/history/{}.json",
                date.format("%Y%m%d").to_string()
            );
            info!("zkillboard.com API: {zkb_api}");

            let mut map = reqwest::get(&zkb_api)
                .await?
                .json::<HashMap<i32, String>>()
                .await?;
            info!("Received {} killmails from zkillboard.com", map.len());

            let saved = reqwest::get(&zkbinfo_get_saved_api)
                .await?
                .json::<Vec<killmail::Key>>()
                .await?;
            info!("Received {} killmails from zkbinfo", saved.len());

            for id in saved {
                map.remove(&id);
            }
            info!("The rest of killmails to receive {}", map.len());

            for (id, hash) in map {
                let evetech_api = format!(
                    "https://esi.evetech.net/latest/killmails/{id}/{hash}/?datasource=tranquility"
                );
                info!("EVETECH API: {evetech_api}");

                let mut timeout = 10;
                loop {
                    let response = reqwest::get(&evetech_api).await?;
                    if let Ok(killmail) = response.json::<killmail::Killmail>().await {
                        while let Err(what) =
                            client.post(&zkbinfo_save_api).json(&killmail).send().await
                        {
                            error!("{what}");
                            tokio::time::sleep(Duration::from_secs(timeout)).await;
                            warn!("Will wait zkbinfo for {timeout} seconds");
                        }
                        break;
                    } else {
                        if timeout < 300 {
                            timeout += 30;
                        }
                        tokio::time::sleep(Duration::from_secs(timeout)).await;
                        warn!("Will wait evetech for {timeout} seconds");
                    }
                }
            }
            return Ok(());
        }
    }
    Ok(usage(&args[0]))
}

fn usage(app: &String) -> () {
    println!("Usage:\n\t{app} <YYYY-MM-DD>");
}
