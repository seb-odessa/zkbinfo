use env_logger;
use log::{error, info};
use tokio::time::{sleep, Duration};
use websockets::{Frame, WebSocket, WebSocketError};

use std::env;

use lib::killmail::Killmail;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("ZKBINFO_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("ZKBINFO_PORT")
        .unwrap_or_default()
        .parse::<u16>()
        .unwrap_or(8080);
    let api = format!("http://{host}:{port}/killmail/save");
    info!("zkbinfo API url: {api}");

    let wss = "wss://zkillboard.com/websocket/";
    let enable = r#"{"action":"sub","channel":"killstream"}"#;

    let client = reqwest::Client::new();
    info!("Reqwest client created");
    let mut ws = WebSocket::connect(wss).await?;
    info!("Web Socket {:?} created", ws);
    ws.send_text(enable.to_string()).await?;
    info!("Web Socket request sent");

    let mut parts = Vec::new();
    loop {
        let maybe_response = ws.receive().await;
        match maybe_response {
            Ok(response) => {
                if let Frame::Text {
                    payload,
                    continuation,
                    fin,
                } = response
                {
                    parts.push(payload);
                    if !continuation || fin {
                        let json = parts.join("");
                        parts.clear();
                        match serde_json::from_str::<Killmail>(&json) {
                            Ok(killmail) => {
                                info!("killmail_id: {}", killmail.killmail_id);
                                let res = client.post(&api).json(&killmail).send().await;
                                if res.is_err() {
                                    sleep(Duration::from_secs(10)).await;
                                    client.post(&api).json(&killmail).send().await?;
                                }
                            }
                            Err(what) => {
                                error!(": {what}");
                            }
                        }
                    }
                }
            }
            Err(what) => {
                error!("Web Socket {what}");
                if let WebSocketError::ReadError(_) = what {
                    ws.close(None).await?;
                    ws = WebSocket::connect(wss).await?;
                    ws.send_text(enable.to_string()).await?;
                }
            }
        }
    }
}
