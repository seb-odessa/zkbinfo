use super::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
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
pub struct SearchResult {
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
