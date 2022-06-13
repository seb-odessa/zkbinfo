use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use anyhow::anyhow;
use env_logger;
use log::{error, info};
use rusqlite::{named_params, Connection /*, Transaction*/};

use lib::Killmail;
use serde::Serialize;
use std::sync::Mutex;

pub struct AppState {
    stat: Mutex<Stat>,
    connection: Mutex<Connection>,
}
impl AppState {
    pub fn new(connection: Connection) -> Self {
        Self {
            stat: Mutex::new(Stat::default()),
            connection: Mutex::new(connection),
        }
    }
}

#[derive(Serialize, Clone, Default)]
struct Stat {
    received_killmails: u32,
}
impl Responder for Stat {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(actix_web::http::header::ContentType::json())
            .body(body)
    }
}

type Context = web::Data<AppState>;

#[derive(Serialize)]
struct Status {
    code: u32,
    message: String,
}
impl Status {
    pub fn ok() -> Self {
        Self {
            code: 0,
            message: String::from("Success"),
        }
    }
    pub fn from<T: Into<String>>(code: u32, message: T) -> Self {
        Self { code, message: message.into() }
    }
}
impl Responder for Status {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(actix_web::http::header::ContentType::json())
            .body(body)
    }
}

mod api {
    use super::*;

    pub async fn stat(ctx: Context) -> impl Responder {
        if let Some(stat) = ctx.stat.lock().ok() {
            return stat.clone();
        } else {
            return Stat::default();
        }
    }

    //pub async fn stat(ctx: Context, name: web::Path<String>) -> impl Responder {
}

mod killmail {
    use super::*;
    pub async fn save(ctx: Context, json: String) -> impl Responder {
        match serde_json::from_str::<Killmail>(&json) {
            Ok(killmail) => {
                let id = killmail.killmail_id;
                info!("killmail_id {} was parsed", id);
                match ctx.connection.lock() {
                    Ok(conn) => match insert(killmail, &conn) {
                        Ok(_) => {
                            info!("killmail_id {} was inserted", id);
                            if let Some(mut stat) = ctx.stat.lock().ok() {
                                stat.received_killmails += 1;
                            }
                            Status::ok()
                        }
                        Err(what) => {
                            error!("Failed to lock connection: {what}");
                            Status::from(2, format!("{what}"))
                        }
                    },
                    Err(what) => {
                        error!("Failed to lock connection: {what}");
                        Status::from(2, format!("{what}"))
                    }
                }
            }
            Err(what) => {
                error!("Failed to parse killamil: {what}");
                Status::from(1, format!("{what}"))
            }
        }
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let database = "killmail.db";
    info!("The Database path: {database}");
    let connection = create_connection(&database)?;
    let state = AppState::new(connection);
    let context = web::Data::new(state);

    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .service(web::scope("/api").route("/stat", web::get().to(api::stat)))
            .service(web::scope("/killmail").route("/save", web::post().to(killmail::save)))
            .wrap(Logger::default())
    })
    .workers(3)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(|e| anyhow!(e))
}

fn create_connection(url: &str) -> anyhow::Result<Connection> {
    let conn = Connection::open(url)?;
    info!("Connection to the {url} complete.");
    let _ = conn.execute_batch("
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS killmails(
            killmail_id INTEGER NOT NULL PRIMARY KEY,
            killmail_time TEXT NOT NULL,
            solar_system_id INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS killmail_time_idx ON killmails(killmail_time);

        CREATE TABLE IF NOT EXISTS participants(
            killmail_id INTEGER NOT NULL,
            character_id INTEGER,
            corporation_id INTEGER,
            alliance_id INTEGER,
            ship_type_id INTEGER,
            damage INTEGER NOT NULL,
            is_victim INTEGER NOT NULL,
            UNIQUE(killmail_id, character_id, is_victim),
            FOREIGN KEY(killmail_id) REFERENCES killmails(killmail_id)
        );
        CREATE INDEX IF NOT EXISTS participant_idx ON participants(character_id, corporation_id, alliance_id);
    ").map_err(|e| anyhow!(e))?;

    return Ok(conn);
}

fn insert(killmail: Killmail, conn: &Connection) -> anyhow::Result<()> {
    const INSERT_KILLMAIL: &str = r"INSERT OR IGNORE INTO killmails VALUES (
        :killmail_id,
        :killmail_time,
        :solar_system_id)";

    const INSERT_PARTICIPANT: &str = r"INSERT OR IGNORE INTO participants VALUES (
        :killmail_id,
        :character_id,
        :corporation_id,
        :alliance_id,
        :ship_type_id,
        :damage,
        :is_victim)";

    let mut insert_killmail_stmt = conn.prepare(INSERT_KILLMAIL)?;
    let mut insert_participant_stmt = conn.prepare(INSERT_PARTICIPANT)?;

    insert_killmail_stmt.execute(named_params! {
        ":killmail_id": killmail.killmail_id,
        ":killmail_time": killmail.killmail_time,
        ":solar_system_id": killmail.solar_system_id
    })?;

    let victim = killmail.victim;
    insert_participant_stmt.execute(named_params! {
        ":killmail_id": killmail.killmail_id,
        ":character_id": victim.character_id,
        ":corporation_id": victim.corporation_id,
        ":alliance_id": victim.alliance_id,
        ":ship_type_id": victim.ship_type_id,
        ":damage": victim.damage_taken,
        ":is_victim": 1
    })?;

    for attacker in killmail.attackers {
        insert_participant_stmt.execute(named_params! {
            ":killmail_id": killmail.killmail_id,
            ":character_id": attacker.character_id,
            ":corporation_id": attacker.corporation_id,
            ":alliance_id": attacker.alliance_id,
            ":ship_type_id": attacker.ship_type_id,
            ":damage": attacker.damage_done,
            ":is_victim": 0
        })?;
    }

    Ok(())
}
