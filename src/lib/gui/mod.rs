use anyhow::anyhow;
use chrono::NaiveDateTime;
use futures::future::join_all;
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

use lazy_static::lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref CHARACTERS: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
    static ref CORPORATIONS: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
    static ref ALLIANCES: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
}

struct NameProvider;

impl NameProvider {
    fn find_id(name: String, category: SearchCategory) -> Option<i32> {
        match category {
            SearchCategory::Character => {
                if let Ok(map) = CHARACTERS.lock() {
                    return map.get(&name).and_then(|id| Some(*id));
                }
            }
            SearchCategory::Corporation => {
                if let Ok(map) = CORPORATIONS.lock() {
                    return map.get(&name).and_then(|id| Some(*id));
                }
            }
            SearchCategory::Alliance => {
                if let Ok(map) = ALLIANCES.lock() {
                    return map.get(&name).and_then(|id| Some(*id));
                }
            }
            _ => {
                return None;
            }
        }
        return None;
    }

    fn update(result: SearchResult) -> anyhow::Result<()> {
        if let Some(items) = result.characters {
            if let Ok(mut map) = CHARACTERS.lock() {
                for item in items {
                    map.entry(item.name).or_insert(item.id);
                }
            }
        }
        if let Some(items) = result.corporations {
            if let Ok(mut map) = CORPORATIONS.lock() {
                for item in items {
                    map.entry(item.name).or_insert(item.id);
                }
            }
        }
        if let Some(items) = result.alliances {
            if let Ok(mut map) = ALLIANCES.lock() {
                for item in items {
                    map.entry(item.name).or_insert(item.id);
                }
            }
        }

        Ok(())
    }

    pub async fn get_id(name: String, category: SearchCategory) -> anyhow::Result<i32> {
        if let Some(id) = Self::find_id(name.clone(), category.clone()) {
            return Ok(id);
        }
        let sr = SearchResult::from(name.clone(), category.clone()).await?;
        Self::update(sr)?;
        Self::find_id(name.clone(), category)
            .ok_or(format!("Can't find id for {name}"))
            .map_err(|e| anyhow!(e))
    }
}

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
        let id = NameProvider::get_id(name, SearchCategory::Character).await?;
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
        let id = NameProvider::get_id(name, SearchCategory::Corporation).await?;
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
        let id = NameProvider::get_id(name, SearchCategory::Alliance).await?;
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

#[derive(Deserialize, Debug)]
pub struct WhoFormData {
    names: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct WhoIsCharacter {
    character_id: i32,
    character_name: String,
    corporation_id: i32,
    corporation_name: String,
    alliance_id: i32,
    alliance_name: String,
    wins_count: i32,
    losses_count: i32,
    wins_percent: String,
    losses_percent: String,

    damage_dealt: i32,
    damage_received: i32,
    damage_dealt_percent: String,
    damage_received_percent: String,
}
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Wins {
    total_count: i32,
    total_damage: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Losses {
    total_count: i32,
    total_damage: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Activity {
    id: i32,
    wins: Wins,
    losses: Losses,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WhoProps {
    characters: Vec<WhoIsCharacter>,
}
impl WhoProps {
    async fn activity(id: i32) -> anyhow::Result<Activity> {
        let url = format!("http://zkbinfo:8080/api/character/activity/{id}/");
        info!("{url}");
        reqwest::get(&url)
            .await?
            .json::<Activity>()
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn from(data: WhoFormData) -> anyhow::Result<Self> {
        let get_ids_tasks = data
            .names
            .split("\r\n")
            .map(|name| String::from(name))
            .filter(|name| !name.is_empty())
            .map(|name| SearchResult::from(name, SearchCategory::Character))
            .collect::<Vec<_>>();

        let ids = join_all(get_ids_tasks)
            .await
            .into_iter()
            .filter(|search_result| search_result.is_ok())
            .map(|search_result| search_result.unwrap().get_character_id())
            .map(|maybe_id| maybe_id.unwrap_or_default())
            .filter(|id| *id != 0)
            .collect::<Vec<i32>>();

        let get_chars_tasks = join_all(ids.iter().map(|id| Character::from(*id))).await;
        let get_activity_tasks = join_all(ids.iter().map(|id| Self::activity(*id))).await;
        let char_map = ids
            .iter()
            .zip(
                get_chars_tasks
                    .into_iter()
                    .map(|result| result.unwrap_or_default()),
            )
            .collect::<HashMap<&i32, Character>>();

        let activity_map = ids
            .iter()
            .zip(
                get_activity_tasks
                    .into_iter()
                    .map(|result| result.unwrap_or_default()),
            )
            .collect::<HashMap<&i32, Activity>>();

        let mut corp_ids = char_map
            .values()
            .map(|character| character.corporation_id)
            .collect::<Vec<i32>>();
        corp_ids.sort();
        corp_ids.dedup();

        let get_corps_tasks = join_all(corp_ids.iter().map(|id| Corporation::from(*id))).await;
        let corp_map = corp_ids
            .into_iter()
            .zip(
                get_corps_tasks
                    .into_iter()
                    .map(|result| result.unwrap_or_default()),
            )
            .collect::<HashMap<i32, Corporation>>();

        let mut alli_ids = corp_map
            .values()
            .map(|corporation| corporation.alliance_id.unwrap_or_default())
            .filter(|id| *id != 0)
            .collect::<Vec<i32>>();
        alli_ids.sort();
        alli_ids.dedup();

        let get_allis_tasks = join_all(alli_ids.iter().map(|id| Alliance::from(*id))).await;
        let alli_map = alli_ids
            .into_iter()
            .zip(
                get_allis_tasks
                    .into_iter()
                    .map(|result| result.unwrap_or_default()),
            )
            .collect::<HashMap<i32, Alliance>>();

        let mut characters = Vec::new();
        for (id, character) in char_map {
            if character.name.is_empty() {
                continue;
            }
            let corporation_id = character.corporation_id;
            let corporation_name: String = corp_map
                .get(&character.corporation_id)
                .and_then(|corp| Some(corp.name.clone()))
                .unwrap_or_default();

            let alliance_id = character.alliance_id.unwrap_or_default();
            let alliance_name: String = alli_map
                .get(&character.alliance_id.unwrap_or_default())
                .and_then(|corp| Some(corp.name.clone()))
                .unwrap_or_default();

            let activity: Activity = activity_map.get(&id).cloned().unwrap_or_default();
            let total_combats = (activity.wins.total_count + activity.losses.total_count) as f32;
            let wins_percent = if total_combats > 0.0 {
                format!(
                    "{:.2}%",
                    100.0 * activity.wins.total_count as f32 / total_combats
                )
            } else {
                format!("")
            };
            let losses_percent = if total_combats > 0.0 {
                format!(
                    "{:.2}%",
                    100.0 * activity.losses.total_count as f32 / total_combats
                )
            } else {
                format!("")
            };

            let total_damage = (activity.wins.total_damage + activity.losses.total_damage) as f32;
            let damage_dealt_percent = if total_damage > 0.0 {
                format!(
                    "{:.2}%",
                    100.0 * activity.wins.total_damage as f32 / total_damage
                )
            } else {
                format!("")
            };
            let damage_received_percent = if total_damage > 0.0 {
                format!(
                    "{:.2}%",
                    100.0 * activity.losses.total_damage as f32 / total_damage
                )
            } else {
                format!("")
            };

            let character = WhoIsCharacter {
                character_id: *id,
                character_name: character.name,
                corporation_id: corporation_id,
                corporation_name: corporation_name,
                alliance_id: alliance_id,
                alliance_name: alliance_name,
                wins_count: activity.wins.total_count,
                losses_count: activity.losses.total_count,
                wins_percent: wins_percent,
                losses_percent: losses_percent,
                damage_dealt: activity.wins.total_damage,
                damage_received: activity.losses.total_damage,
                damage_dealt_percent: damage_dealt_percent,
                damage_received_percent: damage_received_percent,
            };
            characters.push(character);
        }

        characters.sort_by(|a, b| {
            if a.alliance_name == b.alliance_name {
                if a.corporation_name == b.corporation_name {
                    a.character_name.cmp(&b.character_name)
                } else {
                    a.corporation_name.cmp(&b.corporation_name)
                }
            } else {
                a.alliance_name.cmp(&b.alliance_name)
            }
        });

        Ok(Self { characters })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn name_provider_get_char_id() -> Result<(), String> {
        let id = NameProvider::get_id(String::from("Seb Odessa"), SearchCategory::Character)
            .await
            .map_err(|e| format!("{e}"))?;

        assert_eq!(2114350216, id);
        Ok(())
    }

    #[tokio::test]
    async fn name_provider_get_corp_id() -> Result<(), String> {
        let id = NameProvider::get_id(String::from("SO Corporation"), SearchCategory::Corporation)
            .await
            .map_err(|e| format!("{e}"))?;

        assert_eq!(98573194, id);
        Ok(())
    }

    #[tokio::test]
    async fn name_provider_get_alli_id() -> Result<(), String> {
        let id = NameProvider::get_id(String::from("Train Wreck."), SearchCategory::Alliance)
            .await
            .map_err(|e| format!("{e}"))?;

        assert_eq!(99011258, id);
        Ok(())
    }
}
