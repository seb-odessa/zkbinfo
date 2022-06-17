#[macro_use]
extern crate actix_web;

extern crate serde_json;

use anyhow::anyhow;
use actix_web::{web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

pub type Context<'a>=web::Data<Handlebars<'a>>;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(index)
    })
    .bind("localhost:8088")?
    .run()
    .await
    .map_err(|e| anyhow!(e))
}

//                     let response = reqwest::get(&evetech_api).await?;
// if let Ok(killmail) = response.json::<killmail::Killmail>().await {

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct Name {
    // https://esi.evetech.net/latest/characters/2114350216/portrait/?datasource=tranquility
    // {
    //   "px128x128": "https://images.evetech.net/characters/2114350216/portrait?tenant=tranquility&size=128",
    //   "px256x256": "https://images.evetech.net/characters/2114350216/portrait?tenant=tranquility&size=256",
    //   "px512x512": "https://images.evetech.net/characters/2114350216/portrait?tenant=tranquility&size=512",
    //   "px64x64": "https://images.evetech.net/characters/2114350216/portrait?tenant=tranquility&size=64"
    // }
    name: String
}

#[get("/gui/character/{name}/")]
async fn index(ctx: Context<'_>, param: web::Path<String>) -> HttpResponse {
    let body = ctx.render("index", &Name{name: param.into_inner()}).unwrap();
    HttpResponse::Ok().body(body)
}

