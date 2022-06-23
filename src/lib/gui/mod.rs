use anyhow::anyhow;
use chrono::NaiveDateTime;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::evetech::Alliance;
use crate::evetech::AllianceIcon;
use crate::evetech::Character;
use crate::evetech::CharacterPortrait;
use crate::evetech::Corporation;
use crate::evetech::CorporationIcon;

use crate::evetech::Names;
use crate::evetech::SearchCategory;
use crate::evetech::SearchResult;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CharacterProps {
    character_id: i32,
    character_name: String,
    character_gender: String,
    character_birthday: String,
    character_security_status: String,

    corporation_id: i32,
    alliance_id: i32,

    img_128x128: String,
}

impl CharacterProps {
    pub async fn named(name: String) -> anyhow::Result<Self> {
        let id = SearchResult::from(&name, SearchCategory::Character)
            .await?
            .get_character_id()?;
        Self::from(id).await
    }
    pub async fn from(id: i32) -> anyhow::Result<Self> {
        let character = Character::from(id).await?;
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
            alliance_id: character.alliance_id.unwrap_or_default(),
            img_128x128: portrait.px128x128,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CorporationProps {
    corporation_id: i32,
    corporation_name: String,
    corporation_icon: String,
    corporation_ticker: String,
    corporation_member_count: i32,
    corporation_ceo_id: i32,
    corporation_creator_id: i32,
    corporation_founded: Option<String>,
    corporation_description: Option<String>,
    corporation_home_station_id: Option<i32>,
    corporation_url: Option<String>,
    corporation_war_eligible: Option<bool>,
    alliance_id: Option<i32>,
}

impl CorporationProps {
    pub async fn named(name: String) -> anyhow::Result<Self> {
        let id = SearchResult::from(&name, SearchCategory::Corporation)
            .await?
            .get_corporation_id()?;
        Self::from(id).await
    }

    pub async fn from(id: i32) -> anyhow::Result<Self> {
        let corporation = Corporation::from(id).await?;
        let icons = CorporationIcon::from(id).await?;
        let parse_date = NaiveDateTime::parse_from_str;

        Ok(Self {
            corporation_id: id,
            corporation_name: corporation.name,
            corporation_icon: icons.px128x128,
            corporation_ticker: corporation.ticker,
            corporation_member_count: corporation.member_count,
            corporation_ceo_id: corporation.ceo_id,
            corporation_creator_id: corporation.creator_id,
            corporation_founded: corporation
                .date_founded
                .and_then(|founded| parse_date(&founded, "%Y-%m-%dT%H:%M:%SZ").ok())
                .and_then(|date| Some(date.format("%Y-%m-%d %H:%M:%S").to_string())),
            corporation_description: None,
            // corporation.description
            //     .and_then(|desc| serde_json::from_str(&desc).ok()) + unescape unicode
            corporation_home_station_id: corporation.home_station_id,
            corporation_url: corporation.url,
            corporation_war_eligible: corporation.war_eligible,
            alliance_id: corporation.alliance_id,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AllianceProps {
    alliance_id: i32,
    alliance_name: String,
    alliance_icon: String,
    alliance_ticker: String,

    alliance_creator_id: i32,
    alliance_executor_corporation_id: Option<i32>,
    alliance_founded: Option<String>,
}
impl AllianceProps {
    pub async fn named(name: String) -> anyhow::Result<Self> {
        let id = SearchResult::from(&name, SearchCategory::Alliance)
            .await?
            .get_alliance_id()?;
        Self::from(id).await
    }

    pub async fn from(id: i32) -> anyhow::Result<Self> {
        let alliance = Alliance::from(id).await?;
        let icons = AllianceIcon::from(id).await?;
        let parse_date = NaiveDateTime::parse_from_str;

        Ok(Self {
            alliance_id: id,
            alliance_name: alliance.name,
            alliance_icon: icons.px128x128,
            alliance_ticker: alliance.ticker,
            alliance_creator_id: alliance.creator_id,
            alliance_executor_corporation_id: alliance.executor_corporation_id,
            alliance_founded: parse_date(&alliance.date_founded, "%Y-%m-%dT%H:%M:%SZ")
                .ok()
                .and_then(|date| Some(date.format("%Y-%m-%d %H:%M:%S").to_string())),
        })
    }
}

fn query_all_ids(killmails: &Vec<Killmail>) -> Vec<i32> {
    let mut ids = Vec::with_capacity(5 * killmails.len());
    for row in killmails {
        if let Some(character_id) = row.character_id {
            ids.push(character_id);
        }
        if let Some(corporation_id) = row.corporation_id {
            ids.push(corporation_id);
        }
        if let Some(alliance_id) = row.alliance_id {
            ids.push(alliance_id);
        }
        if let Some(ship_type_id) = row.ship_type_id {
            ids.push(ship_type_id);
        }
        ids.push(row.solar_system_id);
    }
    ids.sort();
    ids.dedup();
    return ids;
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Killmail {
    killmail_id: i32,
    character_id: Option<i32>,
    corporation_id: Option<i32>,
    alliance_id: Option<i32>,
    ship_type_id: Option<i32>,
    damage: i32,
    is_victim: i32,
    solar_system_id: i32,
    killmail_time: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LostProps {
    name: String,
    ship_name: String,
    killmails: Vec<Killmail>,
    ids: String,
}

impl LostProps {
    pub async fn from(id: i32, ship_id: i32, category: SearchCategory) -> anyhow::Result<Self> {
        let category_path = SearchCategory::category(&category);
        let url = format!("http://zkbinfo:8080/api/{category_path}/{id}/lost/{ship_id}/");
        info!("{url}");
        let killmails = reqwest::get(&url)
            .await?
            .json::<Vec<Killmail>>()
            .await
            .map_err(|e| anyhow!(e))?;

        let names = Names::from(&vec![id, ship_id]).await?;
        let ids = query_all_ids(&killmails);
        Ok(Self {
            name: names.get_name(category, id)?,
            ship_name: names.get_name(SearchCategory::InventoryType, ship_id)?,
            killmails: killmails,
            ids: serde_json::to_string(&ids)?,
        })
    }
}
