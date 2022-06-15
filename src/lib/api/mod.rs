use actix_web::{web, HttpRequest, HttpResponse, Responder};
use rusqlite::Connection;
use serde::Serialize;
use std::sync::Mutex;
use chrono::NaiveDate;
use log::{error,warn};

use crate::killmail;
use crate::database;

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
        format!(r#"{{ "message": {} }}"#, message.into())
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
#[derive(Serialize, Clone, Default)]
pub struct Stat {
    received_killmails: u32,
}
impl Stat {
    pub fn increase_killmail_count(&mut self) {
        self.received_killmails += 1;
    }
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
pub async fn statistic(ctx: web::Data<AppState>) -> impl Responder {
    if let Some(stat) = ctx.stat.lock().ok() {
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
    match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(date) => {
            match ctx.connection.lock() {
                Ok(conn) => {
                    match database::select_ids_by_date(&conn, &date) {
                        Ok(vec) => {
                            let json = serde_json::to_string(&vec).unwrap();
                            format!("{json}")
                        }
                        Err(what) => {
                            error!("Failed to select ids from DB: {what}");
                            Status::json(format!("{what}"))
                        }
                    }
                }
                Err(what) => {
                    error!("Failed to lock connection: {what}");
                    Status::json(format!("{what}"))
                }
            }
        },
        Err(what) => {
            warn!("Can't parse date '{date}' due to '{what}'");
            Status::json(format!("Can't parse date '{date}' due to '{what}'"))
        }
    }

}

/******************************************************************************/