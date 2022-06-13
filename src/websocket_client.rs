use env_logger;
use log::{error, info};
use websockets::{Frame, WebSocket, WebSocketError};

use lib::Killmail;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = "http://localhost:8080/killmail/save";
    let wss = "wss://zkillboard.com/websocket/";
    let enable = r#"{"action":"sub","channel":"killstream"}"#;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
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
                    if  !continuation || fin {
                        let json = parts.join("");
                        parts.clear();
                        match serde_json::from_str::<Killmail>(&json) {
                            Ok(killmail) => {
                                info!("killmail_id: {}", killmail.killmail_id);
                                client.post(api).json(&killmail).send().await?;
                            },
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

