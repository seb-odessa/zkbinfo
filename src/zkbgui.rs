#[macro_use]
extern crate handlebars;

use actix_files::Files;
use actix_files::NamedFile;
use actix_web::{get, web, App, Responder, HttpResponse, HttpServer};
use anyhow::anyhow;

use handlebars::Handlebars;
use log::{error, info};
use serde::{Deserialize, Serialize};


use lib::gui::CharacterProps;
use lib::gui::CorporationProps;
use lib::gui::AllianceProps;

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

    handlebars::handlebars_helper!(not_zero: |id: i32| id != 0);

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./public/templates")?;
    handlebars.register_helper("not_zero", Box::new(not_zero));
    let context = web::Data::new(handlebars);

    info!("Try http://{host}:{port}/");
    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .service(Files::new("/css", "./public/css").show_files_listing())
            .service(Files::new("/js", "./public/js").show_files_listing())
            .service(favicon)
            .service(report)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
    .map_err(|e| anyhow!(e))
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    NamedFile::open_async("./public/favicon.ico").await
}

#[get("/gui/{target}/{name}/")]
async fn report(ctx: Context<'_>, path: web::Path<(String, String)>) -> HttpResponse {
    let (target, name) = path.into_inner();
    let body = match target.as_str() {
        "character" => match CharacterProps::from(name).await {
            Ok(prop) => wrapper(ctx, "character", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "corporation" => match CorporationProps::from(name).await {
            Ok(prop) => wrapper(ctx, "corporation", &prop),
            Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
        },
        "alliance" => match AllianceProps::from(name).await {
            Ok(prop) => wrapper(ctx, "alliance", &prop),
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
