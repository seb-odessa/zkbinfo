use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse, Responder};

use chrono::NaiveDate;
use log::{error, info, warn};

use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::database;
use crate::evetech;
use database::QuerySubject;
use database::RawHistory;
use database::RelationType;
use database::SqlitePool;

type Context = web::Data<AppState>;
type Param = web::Path<String>;

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

    pub fn notify_access(&self, id: StatType) {
        if let Ok(mut stat) = self.stat.try_lock() {
            *stat.access_count.entry(id).or_insert(0) += 1;
        }
    }

    pub fn notify(&self, subj: QuerySubject, st: StatType) {
        if let Ok(mut stat) = self.stat.try_lock() {
            match subj {
                QuerySubject::Character => *stat.character.entry(st).or_insert(0) += 1,
                QuerySubject::Corporation => *stat.corporation.entry(st).or_insert(0) += 1,
                QuerySubject::Alliance => *stat.alliance.entry(st).or_insert(0) += 1,
            }
        }
    }
}

#[derive(Serialize, Clone, Eq, PartialEq, Hash)]
pub enum StatType {
    SavedKillmailsCount,
    StatisticAccessedCount,
    SelectKillmailsByDateCount,

    ActivityCount,
    ActivityHourlyCount,

    FriendsCharacterCount,
    FriendsCorporationCount,
    FriendsAllianceCount,

    EnemiesCharacterCount,
    EnemiesCorporationCount,
    EnemiesAllianceCount,

    CharacterActivityCount,
    CorporationActivityCount,
    AllianceActivityCount,
}

#[derive(Serialize, Clone, Default)]
pub struct Stat {
    access_count: HashMap<StatType, usize>,
    character: HashMap<StatType, usize>,
    corporation: HashMap<StatType, usize>,
    alliance: HashMap<StatType, usize>,
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

pub mod character {
    use super::*;
    const SUBJECT: QuerySubject = QuerySubject::Character;

    pub async fn activity(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::ActivityCount);

        activity_wrapper(ctx, id, SUBJECT)
    }

    pub async fn activity_hourly(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::ActivityHourlyCount);

        activity_hourly_wrapper(ctx, id, SUBJECT)
    }

    pub async fn friends_char(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::FriendsCharacterCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::FriendsChar)
    }

    pub async fn enemies_char(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::EnemiesCharacterCount);

        relations_wrapper(ctx, id, QuerySubject::Character, RelationType::EnemiesChar)
    }

    pub async fn friends_corp(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::FriendsCorporationCount);

        relations_wrapper(ctx, id, QuerySubject::Character, RelationType::FriendsCorp)
    }

    pub async fn enemies_corp(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::EnemiesCorporationCount);

        relations_wrapper(ctx, id, QuerySubject::Character, RelationType::EnemiesCorp)
    }

    pub async fn friends_alli(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::FriendsAllianceCount);

        relations_wrapper(ctx, id, QuerySubject::Character, RelationType::FriendsAlli)
    }

    pub async fn enemies_alli(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::EnemiesAllianceCount);

        relations_wrapper(ctx, id, QuerySubject::Character, RelationType::EnemiesAlli)
    }
}

pub mod corporation {
    use super::*;
    const SUBJECT: QuerySubject = QuerySubject::Corporation;

    pub async fn activity(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::CorporationActivityCount);

        activity_wrapper(ctx, id, SUBJECT)
    }

    pub async fn activity_hourly(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::ActivityHourlyCount);

        activity_hourly_wrapper(ctx, id, SUBJECT)
    }

    pub async fn friends_char(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::FriendsCharacterCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::FriendsChar)
    }

    pub async fn enemies_char(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::EnemiesCharacterCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::EnemiesChar)
    }

    pub async fn friends_corp(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::FriendsCorporationCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::FriendsCorp)
    }

    pub async fn enemies_corp(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::EnemiesCorporationCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::EnemiesCorp)
    }

    pub async fn friends_alli(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::FriendsAllianceCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::FriendsAlli)
    }

    pub async fn enemies_alli(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::EnemiesAllianceCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::EnemiesAlli)
    }
}

pub mod alliance {
    use super::*;
    const SUBJECT: QuerySubject = QuerySubject::Alliance;

    pub async fn activity(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::AllianceActivityCount);

        activity_wrapper(ctx, id, SUBJECT)
    }

    pub async fn activity_hourly(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::ActivityHourlyCount);

        activity_hourly_wrapper(ctx, id, SUBJECT)
    }

    pub async fn friends_char(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::FriendsCharacterCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::FriendsChar)
    }

    pub async fn enemies_char(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::EnemiesCharacterCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::EnemiesChar)
    }

    pub async fn friends_corp(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::FriendsCorporationCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::FriendsCorp)
    }

    pub async fn enemies_corp(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::EnemiesCorporationCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::EnemiesCorp)
    }

    pub async fn friends_alli(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::FriendsAllianceCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::FriendsAlli)
    }

    pub async fn enemies_alli(ctx: Context, id: Param) -> impl Responder {
        ctx.notify(SUBJECT, StatType::EnemiesAllianceCount);

        relations_wrapper(ctx, id, SUBJECT, RelationType::EnemiesAlli)
    }
}

/******************************************************************************/
pub async fn statistic(ctx: Context) -> impl Responder {
    ctx.notify_access(StatType::StatisticAccessedCount);

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
pub async fn saved_ids(ctx: Context, date: Param) -> impl Responder {
    ctx.notify_access(StatType::SelectKillmailsByDateCount);

    let json = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(date) => {
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

fn save_impl(ctx: Context, json: String) -> anyhow::Result<i32> {
    let killmail = serde_json::from_str::<evetech::Killmail>(&json)?;
    let id = killmail.killmail_id;
    let pool = ctx.get_pool();
    let conn = pool.get()?;
    let _ = database::insert(&conn, killmail)?;
    Ok(id)
}

pub async fn save(ctx: Context, json: String) -> impl Responder {
    ctx.notify_access(StatType::SavedKillmailsCount);

    match save_impl(ctx, json) {
        Ok(id) => {
            info!("killmail {} saved in the database", id);
            Status::from(format!("Success"))
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
    solar_systems: HashMap<i32, usize>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct Losses {
    killmails: Vec<i32>,
    total_damage: i32,
    ships: HashMap<i32, usize>,
    solar_systems: HashMap<i32, usize>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct Activity {
    id: i32,
    wins: Wins,
    losses: Losses,
}
impl Activity {
    pub fn from(id: i32, rows: Vec<RawHistory>) -> Self {
        let mut report = Activity::default();
        report.id = id;
        report.wins.killmails.reserve(rows.len());
        report.losses.killmails.reserve(rows.len());
        for row in rows {
            if row.is_victim {
                report.losses.killmails.push(row.killmail_id);
                report.losses.total_damage += row.damage;
                *report
                    .losses
                    .solar_systems
                    .entry(row.solar_system_id)
                    .or_insert(0) += 1;
                if let Some(id) = row.ship_type_id {
                    *report.losses.ships.entry(id).or_insert(0) += 1;
                }
            } else {
                report.wins.killmails.push(row.killmail_id);
                report.wins.total_damage += row.damage;
                *report
                    .wins
                    .solar_systems
                    .entry(row.solar_system_id)
                    .or_insert(0) += 1;
                if let Some(id) = row.ship_type_id {
                    *report.wins.ships.entry(id).or_insert(0) += 1;
                }
            }
        }
        return report;
    }
}

fn activity_impl(ctx: Context, id: Param, sbj: QuerySubject) -> anyhow::Result<Activity> {
    let id = id.parse::<i32>()?;
    let pool = ctx.get_pool();
    let conn = pool.get()?;
    let rows = database::history(&conn, id, sbj)?;

    Ok(Activity::from(id, rows))
}

fn activity_wrapper(ctx: Context, id: Param, sbj: QuerySubject) -> impl Responder {
    let json = match activity_impl(ctx, id, sbj) {
        Ok(report) => serde_json::to_string(&report).unwrap(),
        Err(what) => Status::json(format!("{what}")),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json)
}

/******************************************************************************/
fn relation_impl(
    ctx: Context,
    id: Param,
    sbj: QuerySubject,
    rel: RelationType,
) -> anyhow::Result<HashMap<i32, usize>> {
    let id = id.parse::<i32>()?;
    let pool = ctx.get_pool();
    let conn = pool.get()?;
    let map = database::relations(&conn, id, sbj, rel)?
        .into_iter()
        .collect::<HashMap<i32, usize>>();
    Ok(map)
}

fn relations_wrapper(
    ctx: Context,
    id: Param,
    sbj: QuerySubject,
    rel: RelationType,
) -> impl Responder {
    let json = match relation_impl(ctx, id, sbj, rel) {
        Ok(report) => serde_json::to_string(&report).unwrap(),
        Err(what) => Status::json(format!("{what}")),
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json)
}

/******************************************************************************/

fn activity_hourly_impl(
    ctx: Context,
    id: Param,
    sbj: QuerySubject,
) -> anyhow::Result<HashMap<i32, usize>> {
    let id = id.parse::<i32>()?;
    let pool = ctx.get_pool();
    let conn = pool.get()?;
    let mut map = database::activity(&conn, id, sbj)?
        .into_iter()
        .collect::<HashMap<i32, usize>>();

    for hour in 0..24 {
        if !map.contains_key(&hour) {
            map.insert(hour, 0);
        }
    }
    Ok(map)
}

fn activity_hourly_wrapper(ctx: Context, id: Param, sbj: QuerySubject) -> impl Responder {
    let json = match activity_hourly_impl(ctx, id, sbj) {
        Ok(report) => serde_json::to_string(&report).unwrap(),
        Err(what) => Status::json(format!("{what}")),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json)
}
