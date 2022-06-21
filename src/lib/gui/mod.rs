use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::evetech::Character;
use crate::evetech::CharacterPortrait;
use crate::evetech::Corporation;
use crate::evetech::CorporationIcon;
use crate::evetech::Alliance;
use crate::evetech::AllianceIcon;

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
    pub async fn from(name: String) -> anyhow::Result<Self> {
        let id = SearchResult::from(&name, SearchCategory::Character)
            .await?
            .get_character_id()?;

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
    pub async fn from(name: String) -> anyhow::Result<Self> {
        let id = SearchResult::from(&name, SearchCategory::Corporation)
            .await?
            .get_corporation_id()?;

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
    pub async fn from(name: String) -> anyhow::Result<Self> {
        let id = SearchResult::from(&name, SearchCategory::Alliance)
            .await?
            .get_alliance_id()?;

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
            alliance_founded: parse_date(&alliance.date_founded, "%Y-%m-%dT%H:%M:%SZ").ok()
                                .and_then(|date| Some(date.format("%Y-%m-%d %H:%M:%S").to_string())),
        })
    }
}