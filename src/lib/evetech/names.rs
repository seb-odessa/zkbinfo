use super::*;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct RawName {
    category: String,
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Names {
    pub values: HashMap<SearchCategory, HashMap<i32, String>>,
}
impl Names {
    pub async fn from(ids: &Vec<i32>) -> anyhow::Result<Self> {
        let url = format!("{EVE_TECH_ROOT}/universe/names/?{EVE_TECH_SERVER}");
        info!("{url}");

        let mut unique = ids.clone();
        unique.sort();
        unique.dedup();

        let client = reqwest::Client::new();
        let raw_names = client
            .post(&url)
            .json(&unique)
            .send()
            .await?
            .json::<Vec<RawName>>()
            .await?;
        let mut values = HashMap::new();

        for raw in raw_names.into_iter() {
            let category = SearchCategory::from(&raw.category).ok_or(anyhow!("Not a Category"))?;
            values
                .entry(category)
                .or_insert(HashMap::new())
                .entry(raw.id)
                .or_insert(raw.name);
        }
        Ok(Self { values })
    }

    pub fn get_name(&self, category: SearchCategory, id: i32) -> anyhow::Result<String> {
        let names = self
            .values
            .get(&category)
            .ok_or(anyhow!("Category {:?} not found", category))?;
        names
            .get(&id)
            .ok_or(anyhow!("Id {id} not found in {:?}", category))
            .map(|name| name.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn from() -> Result<(), String> {
        let names = Names::from(&vec![2114350216, 3756])
            .await
            .map_err(|e| format!("{e}"))?;

        {
            let name = names.get_name(SearchCategory::Character, 2114350216).map_err(|e| format!("{e}"))?;
            assert_eq!(name, String::from("Seb Odessa"));
        }
        {
            let name = names.get_name(SearchCategory::InventoryType, 3756).map_err(|e| format!("{e}"))?;
            assert_eq!(name, String::from("Gnosis"));
        }

        Ok(())
    }
}
