#[macro_use]
extern crate actix_web;

extern crate serde_json;

use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::anyhow;
use handlebars::Handlebars;
use log::{error, info};
use serde::{Deserialize, Serialize};

pub type Context<'a> = web::Data<Handlebars<'a>>;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(character)
    })
    .bind("localhost:8088")?
    .run()
    .await
    .map_err(|e| anyhow!(e))
}

const EVE_TECH_ROOT: &str = "https://esi.evetech.net/latest";
const EVE_TECH_SERVER: &str = "datasource=tranquility";
const EVE_TECH_SEARCH: &str = "language=en&strict=true";

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
enum SearchCategory {
    Agent,
    Alliance,
    Character,
    Constellation,
    Corporation,
    Faction,
    InventoryType,
    Region,
    SolarSystem,
    Station,
}
impl SearchCategory {
    pub fn category(category: SearchCategory) -> &'static str {
        match category {
            SearchCategory::Agent => "categories=agent",
            SearchCategory::Alliance => "categories=alliance",
            SearchCategory::Character => "categories=character",
            SearchCategory::Constellation => "categories=constellation",
            SearchCategory::Corporation => "categories=corporation",
            SearchCategory::Faction => "categories=faction",
            SearchCategory::InventoryType => "categories=inventory_type",
            SearchCategory::Region => "categories=region",
            SearchCategory::SolarSystem => "categories=solar_system",
            SearchCategory::Station => "categories=station",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct SearchResult {
    agent: Option<Vec<i32>>,
    alliance: Option<Vec<i32>>,
    character: Option<Vec<i32>>,
    constellation: Option<Vec<i32>>,
    corporation: Option<Vec<i32>>,
    faction: Option<Vec<i32>>,
    inventory_type: Option<Vec<i32>>,
    region: Option<Vec<i32>>,
    solar_system: Option<Vec<i32>>,
    station: Option<Vec<i32>>,
}
impl SearchResult {
    pub async fn from(name: &String, category: SearchCategory) -> anyhow::Result<Self> {
        let name = urlencoding::encode(name);
        let category = SearchCategory::category(category);
        let url = format!(
            "{EVE_TECH_ROOT}/search/?{category}&{EVE_TECH_SERVER}&{EVE_TECH_SEARCH}&search={name}"
        );
        info!("{url}");
        reqwest::get(&url)
            .await?
            .json::<Self>()
            .await
            .map_err(|e| anyhow!(e))
    }

    pub fn get_character_id(&self) -> anyhow::Result<i32> {
        self.character
            .iter()
            .next()
            .and_then(|ids| ids.iter().next())
            .and_then(|id| Some(*id))
            .ok_or(anyhow!("Character was not found"))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct CharacterPortrait {
    px128x128: String,
    px256x256: String,
    px512x512: String,
    px64x64: String,
}
impl CharacterPortrait {
    pub async fn from(id: i32) -> anyhow::Result<Self> {
        let url = format!("{EVE_TECH_ROOT}/characters/{id}/portrait/?{EVE_TECH_SERVER}");
        info!("{url}");
        reqwest::get(&url)
            .await?
            .json::<Self>()
            .await
            .map_err(|e| anyhow!(e))
    }
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct CharacterProps {
    id: i32,
    name: String,
    img_64x64: String,
    img_128x128: String,
    img_256x256: String,

}
impl CharacterProps {
    pub async fn from(name: String) -> anyhow::Result<Self> {
        let id = SearchResult::from(&name, SearchCategory::Character)
                    .await?
                    .get_character_id()?;

        let portrait = CharacterPortrait::from(id).await?;
        Ok(Self{
            id,
            name,
            img_64x64: portrait.px64x64,
            img_128x128: portrait.px128x128,
            img_256x256: portrait.px256x256,

        })
    }
}

#[get("/gui/character/{name}/")]
async fn character(ctx: Context<'_>, name: web::Path<String>) -> HttpResponse {
    let body = match CharacterProps::from(name.into_inner()).await {
        Ok(prop) => wrapper(ctx, "character", &prop),
        Err(err) => wrapper(ctx, "error", &Error::from(format!("{err}"))),
    };
    HttpResponse::Ok().body(body)
}
