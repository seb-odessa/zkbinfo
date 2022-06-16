use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse, Responder};

use chrono::NaiveDate;
use log::{error, info, warn};

use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::database;
use crate::killmail;
use database::RawHistory;
use database::RelationType;
use database::SqlitePool;

pub struct AppState {
    pub stat: Mutex<Stat>,
    pub pool: SqlitePool,
}
impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            stat: Mutex::new(Stat::default()),
            pool: pool,
        }
    }

    pub fn get_pool(&self) -> SqlitePool {
        self.pool.clone()
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

    pub fn note_get_character_report_count(&self) {
        if let Ok(mut stat) = self.stat.try_lock() {
            stat.get_character_report_count += 1;
        }
    }
}

#[derive(Serialize, Clone, Default)]
pub struct Stat {
    saved_killmails_count: u32,
    stat_access_count: u32,
    select_ids_by_date_count: u32,
    get_character_report_count: u32,
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
    ids: Vec<i32>,
}
pub async fn saved_ids(ctx: web::Data<AppState>, date: web::Path<String>) -> impl Responder {
    let json = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(date) => {
            ctx.note_select_ids_by_date_count();
            let pool = ctx.get_pool();
            let conn = pool.get().unwrap();
            match database::select_ids_by_date(&conn, &date) {
                Ok(vec) => serde_json::to_string(&vec).unwrap(),
                Err(what) => {
                    error!("Failed to select ids from DB: {what}");
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

fn save_impl(ctx: web::Data<AppState>, json: String) -> anyhow::Result<i32> {
    let killmail = serde_json::from_str::<killmail::Killmail>(&json)?;
    let id = killmail.killmail_id;
    let pool = ctx.get_pool();
    let conn = pool.get()?;
    let _ = database::insert(&conn, killmail)?;
    ctx.note_killmail_count();
    Ok(id)
}

pub async fn save(ctx: web::Data<AppState>, json: String) -> impl Responder {
    match save_impl(ctx, json) {
        Ok(id) => {
            info!("killmail {} saved in the database", id);
            Status::ok()
        }
        Err(what) => {
            error!("Failed to select ids from DB: {what}");
            Status::from(format!("{what}"))
        }
    }
}

/******************************************************************************/
#[derive(Debug, Serialize, Clone, Default)]
pub struct Wins {
    killmails: Vec<i32>,
    total_damage: i32,
    ships: HashMap<i32, usize>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct Losses {
    killmails: Vec<i32>,
    total_damage: i32,
    ships: HashMap<i32, usize>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct CharacterReport {
    id: i32,
    wins: Wins,
    losses: Losses,
    solar_systems: HashMap<i32, usize>,
    friends: HashMap<i32, usize>,
    enemies: HashMap<i32, usize>,
    friends_corp: HashMap<i32, usize>,
    enemies_corp: HashMap<i32, usize>,
    friends_alli: HashMap<i32, usize>,
    enemies_alli: HashMap<i32, usize>,
}
impl CharacterReport {
    pub fn from(id: i32, rows: Vec<RawHistory>) -> Self {
        let mut report = CharacterReport::default();
        report.id = id;
        report.wins.killmails.reserve(rows.len());
        report.losses.killmails.reserve(rows.len());
        for row in rows {
            *report.solar_systems.entry(row.solar_system_id).or_insert(0) += 1;
            if row.is_victim {
                report.losses.killmails.push(row.killmail_id);
                report.losses.total_damage += row.damage;
                if let Some(id) = row.ship_type_id {
                    *report.losses.ships.entry(id).or_insert(0) += 1;
                }
            } else {
                report.wins.killmails.push(row.killmail_id);
                report.wins.total_damage += row.damage;
                if let Some(id) = row.ship_type_id {
                    *report.wins.ships.entry(id).or_insert(0) += 1;
                }
            }
        }
        return report;
    }
}

fn character_report_impl(
    ctx: web::Data<AppState>,
    id: web::Path<String>,
) -> anyhow::Result<CharacterReport> {
    ctx.note_get_character_report_count();
    let id = id.parse::<i32>()?;
    let pool = ctx.get_pool();
    let conn = pool.get()?;
    let rows = database::character_history(&conn, id)?;
    let mut report = CharacterReport::from(id, rows);
    report.friends = database::character_relations(&conn, id, RelationType::Friends)?
        .into_iter()
        .collect();

    report.enemies = database::character_relations(&conn, id, RelationType::Enemies)?
        .into_iter()
        .collect();

    report.friends_corp = database::character_relations(&conn, id, RelationType::FriendsCorp)?
        .into_iter()
        .collect();

    report.enemies_corp = database::character_relations(&conn, id, RelationType::EnemiesCorp)?
        .into_iter()
        .collect();

    report.friends_alli = database::character_relations(&conn, id, RelationType::FriendsAlli)?
        .into_iter()
        .collect();

    report.enemies_alli = database::character_relations(&conn, id, RelationType::EnemiesAlli)?
        .into_iter()
        .collect();

    Ok(report)
}

pub async fn character_report(ctx: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let json = match character_report_impl(ctx, id) {
        Ok(report) => serde_json::to_string(&report).unwrap(),
        Err(what) => Status::json(format!("{what}")),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json)
}
