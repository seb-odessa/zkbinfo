use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDate;
use log::{error, warn};
use rusqlite::Connection;
use serde::Serialize;
use std::sync::Mutex;
use std::collections::BTreeMap;

use crate::database;
use crate::killmail;

pub struct AppState {
    pub stat: Mutex<Stat>,
    pub connection: Mutex<Connection>,
}
impl AppState {
    pub fn new(connection: Connection) -> Self {
        Self {
            stat: Mutex::new(Stat::default()),
            connection: Mutex::new(connection),
        }
    }

    pub fn note_killmail_count(&self) {
        if let Ok(mut stat) = self.stat.try_lock() {
            stat.saved_killmails_count += 1;
        }
    }

    pub fn note_stat_access_count(&self) {
        if let Ok(mut stat) = self.stat.try_lock() {
            stat.stat_access_count += 1;
        }
    }

    pub fn note_select_ids_by_date_count(&self) {
        if let Ok(mut stat) = self.stat.try_lock() {
            stat.select_ids_by_date_count += 1;
        }
    }
}

#[derive(Serialize, Clone, Default)]
pub struct Stat {
    saved_killmails_count: u32,
    stat_access_count: u32,
    select_ids_by_date_count: u32,
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

/******************************************************************************/
#[derive(Serialize)]
pub struct Status {
    message: String,
}
impl Status {
    pub fn ok() -> Self {
        Self {
            message: String::from("Success"),
        }
    }
    pub fn from<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
        }
    }
    pub fn json<T: Into<String>>(message: T) -> String {
        format!(r#"{{ "message": "{}" }}"#, message.into())
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

/******************************************************************************/
pub async fn statistic(ctx: web::Data<AppState>) -> impl Responder {
    ctx.note_stat_access_count();
    if let Ok(stat) = ctx.stat.try_lock() {
        return stat.clone();
    } else {
        return Stat::default();
    }
}
/******************************************************************************/
#[derive(Serialize, Clone, Default)]
pub struct KillmailIds {
    ids: Vec<killmail::Key>,
}
pub async fn saved_ids(ctx: web::Data<AppState>, date: web::Path<String>) -> impl Responder {
    let json = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(date) => {
            ctx.note_select_ids_by_date_count();
            match ctx.connection.lock() {
                Ok(conn) => match database::select_ids_by_date(&conn, &date) {
                    Ok(vec) => serde_json::to_string(&vec).unwrap(),
                    Err(what) => {
                        error!("Failed to select ids from DB: {what}");
                        Status::json(format!("{what}"))
                    }
                },
                Err(what) => {
                    error!("Failed to lock connection: {what}");
                    Status::json(format!("{what}"))
                }
            }
        }
        Err(what) => {
            warn!("Can't parse date '{date}' due to '{what}'");
            Status::json(format!("Can't parse date '{date}' due to '{what}'"))
        }
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json)
}

/******************************************************************************/

#[derive(Serialize, Clone, Default)]
pub struct Wins {
    killmails: Vec<i32>,
    total_damage: i32,
    ships: BTreeMap<i32, usize>,
}

#[derive(Serialize, Clone, Default)]
pub struct Losses {
    killmails: Vec<i32>,
    total_damage: i32,
    ships: BTreeMap<i32, usize>,
}

#[derive(Serialize, Clone, Default)]
pub struct CharacterReport {
    id: i32,
    wins: Wins,
    losses: Losses,
}
impl CharacterReport {
    pub fn from(id: i32, rows: Vec<database::RawHistory>) -> Self {
        let mut report = CharacterReport::default();
        report.id = id;
        report.wins.killmails.reserve(rows.len());
        report.losses.killmails.reserve(rows.len());
        for row in rows {
            if row.is_victim {
                report.losses.killmails.push(row.killmail_id);
                report.losses.total_damage += row.damage;
                if let Some(count) = report.losses.ships.get_mut(&row.ship_type_id) {
                    *count += 1;
                }
            } else {
                report.wins.killmails.push(row.killmail_id);
                report.wins.total_damage += row.damage;
                if let Some(count) = report.wins.ships.get_mut(&row.ship_type_id) {
                    *count += 1;
                }
            }
        }
        return report;
    }
}

pub async fn character_report(
    ctx: web::Data<AppState>,
    character: web::Path<String>,
) -> impl Responder {
    let json = if let Ok(id) = character.parse::<i32>() {
        match ctx.connection.lock() {
            Ok(conn) => match database::character_history(&conn, id) {
                Ok(rows) => {
                    serde_json::to_string(&CharacterReport::from(id, rows)).unwrap()
                }
                Err(what) => {
                    error!("Failed to select ids from DB: {what}");
                    Status::json(format!("{what}"))
                }
            },
            Err(what) => {
                error!("Failed to lock connection: {what}");
                Status::json(format!("{what}"))
            }
        }
    } else {
        warn!("Can't parse '{character}' to i32");
        Status::json(format!("Can't parse '{character}' to i32"))
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json)
}
