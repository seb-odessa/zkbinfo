#[macro_use]
extern crate actix_web;

use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::anyhow;
use handlebars::Handlebars;
use log::{error, info};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use lib::evetech::Character;
use lib::evetech::Corporation;
use lib::evetech::CharacterPortrait;
use lib::evetech::SearchCategory;
use lib::evetech::SearchResult;

use std::env;

pub type Context<'a> = web::Data<Handlebars<'a>>;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let host = env::var("ZKBGUI_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("ZKBGUI_PORT")
        .unwrap_or_default()
        .parse::<u16>()
        .unwrap_or(8088);

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./public/templates")?;
    let context = web::Data::new(handlebars);

    info!("Try http://{host}:{port}/");
    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .service(Files::new("/css", "./public/css").show_files_listing())
            .service(Files::new("/js", "./public/js").show_files_listing())
            .service(report)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
    .map_err(|e| anyhow!(e))
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct Error {
    status_code: i32,
    error: String,
}
impl Error {
    pub fn from(error: String) -> Self {
        Error {
            status_code: 0,
            error,
        }
    }
}

fn wrapper<T: Serialize>(ctx: Context<'_>, template: &str, obj: &T) -> String {
    match ctx.render(template, &obj) {
        Ok(ok_body) => ok_body,
        Err(what) => {
            error!("{what}");
            let error = Error {
                status_code: 0,
                error: format!("{what}"),
            };
            match ctx.render("error", &error) {
                Ok(error_body) => error_body,
                Err(error) => {
                    error!("{error}");
                    format!("'{error}' : '{what}'")
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CharacterProps {
    character_id: i32,
    character_name: String,
    character_gender: String,
    character_birthday: String,
    character_security_status: String,

    corporation_id: i32,
    corporation_name: String,
    corporation_ticker: String,
    corporation_members: i32,


    // pub alliance_id: Option<i32>,
    // pub faction_id: Option<i32>,

    img_64x64: String,
    img_128x128: String,
    img_256x256: String,
}
impl CharacterProps {
    pub async fn from(name: String) -> anyhow::Result<Self> {
        let id = SearchResult::from(&name, SearchCategory::Character)
            .await?
            .get_character_id()?;

        let character = Character::from(id).await?;
        let corporation = Corporation::from(character.corporation_id).await?;
        let portrait = CharacterPortrait::from(id).await?;
        let parse_date = NaiveDateTime::parse_from_str;
        let birthday = parse_date(&character.birthday, "%Y-%m-%dT%H:%M:%SZ")?;

        Ok(Self {
            character_id: id,
            character_name: character.name,
            character_gender: character.gender,
            character_birthday: birthday.format("%Y-%m-%d %H:%M:%S").to_string(),
            character_security_status: format!("{:.2}", character.security_status),

            corporation_id: character.corporation_id,
            corporation_name: corporation.name,
            corporation_ticker: corporation.ticker,
            corporation_members: corporation.member_count,

            img_64x64: portrait.px64x64,
            img_128x128: portrait.px128x128,
            img_256x256: portrait.px256x256,
        })
    }
}

#[get("/gui/character/{name}/")]
async fn report(ctx: Context<'_>, name: web::Path<String>) -> HttpResponse {
    let body = match CharacterProps::from(name.into_inner()).await {
        Ok(prop) => wrapper(ctx, "character", &prop),
        Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
    };
    HttpResponse::Ok().body(body)
}
