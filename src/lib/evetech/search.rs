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
    pub id: i32,
    pub name: String
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct SearchResult {
    pub agents: Option<Vec<EveItem>>,
    pub alliances: Option<Vec<EveItem>>,
    pub characters: Option<Vec<EveItem>>,
    pub constellations: Option<Vec<EveItem>>,
    pub corporations: Option<Vec<EveItem>>,
    pub factions: Option<Vec<EveItem>>,
    pub inventory_types: Option<Vec<EveItem>>,
    pub regions: Option<Vec<EveItem>>,
    pub systems: Option<Vec<EveItem>>,
    pub stations: Option<Vec<EveItem>>,
}
impl SearchResult {
    pub async fn from(name: String) -> anyhow::Result<Self> {
        let url = format!("{EVE_TECH_ROOT}/universe/ids/?{EVE_TECH_SERVER}");
        info!("{url}");
        let query = vec!(name.clone());
        reqwest::Client::new()
            .post(&url)
            .json(&query)
            .send()
            .await?
            .json::<Self>()
            .await
            .map_err(|e| anyhow!(e))

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn search_character() -> Result<(), String> {
        let name = String::from("Seb Odessa");
        let search = SearchResult::from(name)
            .await
            .map_err(|e| format!("{e}"))?;
        let items = search.characters
            .ok_or("The SearchResult::charaters is None")?;
        let item = items
            .into_iter()
            .next()
            .ok_or("The SearchResult::charaters is empty")?;
        assert_eq!(2114350216, item.id);
        Ok(())
    }

    #[tokio::test]
    async fn search_corporation() -> Result<(), String> {
        let name = String::from("SO Corporation");
        let search = SearchResult::from(name)
            .await
            .map_err(|e| format!("{e}"))?;
        let items = search.corporations
            .ok_or("The SearchResult::corporations is None")?;
        let item = items
            .into_iter()
            .next()
            .ok_or("The SearchResult::corporations is empty")?;
        assert_eq!(98573194, item.id);
        Ok(())
    }

    #[tokio::test]
    async fn search_alliance() -> Result<(), String> {
        let name = String::from("Train Wreck.");
        let search = SearchResult::from(name)
            .await
            .map_err(|e| format!("{e}"))?;
        let items = search.alliances
            .ok_or("The SearchResult::alliances is None")?;
        let item = items
            .into_iter()
            .next()
            .ok_or("The SearchResult::alliances is empty")?;
        assert_eq!(99011258, item.id);
        Ok(())
    }
}
