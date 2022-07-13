use super::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
pub enum SearchCategory {
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
    pub fn category(category: &SearchCategory) -> &'static str {
        match category {
            SearchCategory::Agent => "agent",
            SearchCategory::Alliance => "alliance",
            SearchCategory::Character => "character",
            SearchCategory::Constellation => "constellation",
            SearchCategory::Corporation => "corporation",
            SearchCategory::Faction => "faction",
            SearchCategory::InventoryType => "inventory_type",
            SearchCategory::Region => "region",
            SearchCategory::SolarSystem => "solar_system",
            SearchCategory::Station => "station",
        }
    }
    pub fn from(category: &str) -> Option<SearchCategory> {
        match category {
            "agent" => Some(SearchCategory::Agent),
            "alliance" => Some(SearchCategory::Alliance),
            "character" => Some(SearchCategory::Character),
            "constellation" => Some(SearchCategory::Constellation),
            "corporation" => Some(SearchCategory::Corporation),
            "faction" => Some(SearchCategory::Faction),
            "inventory_type" => Some(SearchCategory::InventoryType),
            "region" => Some(SearchCategory::Region),
            "solar_system" => Some(SearchCategory::SolarSystem),
            "station" => Some(SearchCategory::Station),
            _ => None
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct EveItem {
    id: i32,
    name: String
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct SearchResult {
    agents: Option<Vec<EveItem>>,
    alliances: Option<Vec<EveItem>>,
    characters: Option<Vec<EveItem>>,
    constellations: Option<Vec<EveItem>>,
    corporations: Option<Vec<EveItem>>,
    factions: Option<Vec<EveItem>>,
    inventory_types: Option<Vec<EveItem>>,
    regions: Option<Vec<EveItem>>,
    systems: Option<Vec<EveItem>>,
    stations: Option<Vec<EveItem>>,
}
impl SearchResult {
    pub async fn from(name: String, _category: SearchCategory) -> anyhow::Result<Self> {
        let url = format!("{EVE_TECH_ROOT}/universe/ids/?{EVE_TECH_SERVER}");
        info!("{url}");
        let query = vec!(name);
        reqwest::Client::new()
            .post(&url)
            .json(&query)
            .send()
            .await?
            .json::<Self>()
            .await
            .map_err(|e| anyhow!(e))

    }

    pub fn get_character_id(&self) -> anyhow::Result<i32> {
        self.characters
            .iter()
            .next()
            .and_then(|items| items.iter().next())
            .and_then(|item| Some(item.id))
            .ok_or(anyhow!("Character was not found"))
    }

    pub fn get_corporation_id(&self) -> anyhow::Result<i32> {
        self.corporations
            .iter()
            .next()
            .and_then(|items| items.iter().next())
            .and_then(|item| Some(item.id))
            .ok_or(anyhow!("Corporation was not found"))
    }

    pub fn get_alliance_id(&self) -> anyhow::Result<i32> {
        self.alliances
            .iter()
            .next()
            .and_then(|items| items.iter().next())
            .and_then(|item| Some(item.id))
            .ok_or(anyhow!("Alliance was not found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn from() -> Result<(), String> {
        let name = String::from("Seb Odessa");
        let search = SearchResult::from(name, SearchCategory::Character)
            .await
            .map_err(|e| format!("{e}"))?;
        let items = search
            .characters
            .ok_or("The SearchResult::charater is None")?;
        let item = items
            .into_iter()
            .next()
            .ok_or("The SearchResult::charater is empty")?;
        assert_eq!(2114350216, item.id);
        Ok(())
    }

    #[tokio::test]
    async fn get_character_id() -> Result<(), String> {
        let name = String::from("Seb Odessa");
        let search = SearchResult::from(name, SearchCategory::Character)
            .await
            .map_err(|e| format!("{e}"))?;

        let id = search.get_character_id().map_err(|e| format!("{e}"))?;

        assert_eq!(2114350216, id);
        Ok(())
    }
}
