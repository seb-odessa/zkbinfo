use actix_files::Files;
use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::anyhow;

use handlebars::Handlebars;
use log::{error, info};
use serde::{Deserialize, Serialize};

use lib::evetech::SearchCategory;
use lib::gui::AllianceProps;
use lib::gui::CharacterProps;
use lib::gui::CorporationProps;
use lib::gui::LostProps;
use lib::gui::WhoFormData;
use lib::gui::WhoProps;

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
            .service(favicon)
            .service(who)
            .service(who_report)
            .service(report)
            .service(report_by_id)
            .service(lost_ships)
    })
    .workers(6)
    .bind((host.as_str(), port))?
    .run()
    .await
    .map_err(|e| anyhow!(e))
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    NamedFile::open_async("./public/favicon.ico").await
}

#[get("/gui/who/")]
async fn who(ctx: Context<'_>) -> HttpResponse {
    let body = wrapper(ctx, "who", &{});
    HttpResponse::Ok().body(body)
}

#[post("/gui/who/report/")]
async fn who_report(ctx: Context<'_>, query: web::Form<WhoFormData>) -> HttpResponse {
    info!("{:?}", query);
    let body = match WhoProps::from(query.into_inner()).await {
        Ok(prop) => wrapper(ctx, "report", &prop),
        Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
    };
    HttpResponse::Ok().body(body)
}

#[get("/gui/{target}/{name}/")]
async fn report(ctx: Context<'_>, path: web::Path<(String, String)>) -> HttpResponse {
    let (target, name) = path.into_inner();
    let body = match target.as_str() {
        "character" => match CharacterProps::named(name).await {
            Ok(prop) => wrapper(ctx, "character", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "corporation" => match CorporationProps::named(name).await {
            Ok(prop) => wrapper(ctx, "corporation", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "alliance" => match AllianceProps::named(name).await {
            Ok(prop) => wrapper(ctx, "alliance", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        _ => wrapper(ctx, "error", &Error::from(format!("Unknown Target"))),
    };
    HttpResponse::Ok().body(body)
}

#[get("/gui/{target}/id/{id}/")]
async fn report_by_id(ctx: Context<'_>, path: web::Path<(String, i32)>) -> HttpResponse {
    let (target, id) = path.into_inner();
    let body = match target.as_str() {
        "character" => match CharacterProps::from(id).await {
            Ok(prop) => wrapper(ctx, "character", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "corporation" => match CorporationProps::from(id).await {
            Ok(prop) => wrapper(ctx, "corporation", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "alliance" => match AllianceProps::from(id).await {
            Ok(prop) => wrapper(ctx, "alliance", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        _ => wrapper(ctx, "error", &Error::from(format!("Unknown Target"))),
    };
    HttpResponse::Ok().body(body)
}

#[get("/gui/{target}/{name}/lost/{ship}/")]
async fn lost_ships(ctx: Context<'_>, path: web::Path<(String, i32, i32)>) -> HttpResponse {
    let (target, id, ship_id) = path.into_inner();
    let body = match target.as_str() {
        "character" => match LostProps::from(id, ship_id, SearchCategory::Character).await {
            Ok(prop) => wrapper(ctx, "losts", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "corporation" => match LostProps::from(id, ship_id, SearchCategory::Corporation).await {
            Ok(prop) => wrapper(ctx, "losts", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "alliance" => match LostProps::from(id, ship_id, SearchCategory::Alliance).await {
            Ok(prop) => wrapper(ctx, "losts", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        _ => wrapper(ctx, "error", &Error::from(format!("Unknown Target"))),
    };

    HttpResponse::Ok().body(body)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct Error {
    error: String,
}
impl Error {
    pub fn from(error: String) -> Self {
        Error { error }
    }
}

fn wrapper<T: Serialize>(ctx: Context<'_>, template: &str, obj: &T) -> String {
    match ctx.render(template, &obj) {
        Ok(ok_body) => ok_body,
        Err(what) => {
            error!("{what}");
            format!("{what}")
        }
    }
}
