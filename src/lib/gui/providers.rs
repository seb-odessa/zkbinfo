use super::*;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref CHARACTERS: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
    static ref CORPORATIONS: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
    static ref ALLIANCES: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
}

pub struct IdProvider;

impl IdProvider {
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

    pub async fn get(name: String, category: SearchCategory) -> anyhow::Result<i32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn id_provider_get_char_id() -> Result<(), String> {
        let id = IdProvider::get(String::from("Seb Odessa"), SearchCategory::Character)
            .await
            .map_err(|e| format!("{e}"))?;

        assert_eq!(2114350216, id);
        Ok(())
    }

    #[tokio::test]
    async fn id_provider_get_corp_id() -> Result<(), String> {
        let id = IdProvider::get(String::from("SO Corporation"), SearchCategory::Corporation)
            .await
            .map_err(|e| format!("{e}"))?;

        assert_eq!(98573194, id);
        Ok(())
    }

    #[tokio::test]
    async fn id_provider_get_alli_id() -> Result<(), String> {
        let id = IdProvider::get(String::from("Train Wreck."), SearchCategory::Alliance)
            .await
            .map_err(|e| format!("{e}"))?;

        assert_eq!(99011258, id);
        Ok(())
    }
}
