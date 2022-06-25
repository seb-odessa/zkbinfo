use super::*;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Alliance {
    pub creator_corporation_id: i32,
    pub creator_id: i32,
    pub date_founded: String,
    pub executor_corporation_id: Option<i32>,
    pub faction_id: Option<i32>,
    pub name: String,
    pub ticker: String,
}
impl Alliance {
    pub async fn from(id: i32) -> anyhow::Result<Self> {
        let url = format!("{EVE_TECH_ROOT}/alliances/{id}/?{EVE_TECH_SERVER}");
        info!("{url}");
        reqwest::get(&url)
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
    async fn from() -> Result<(), String> {
        let corporation = Alliance::from(99003581).await.map_err(|e| format!("{e}"))?;

        assert_eq!(corporation.creator_corporation_id, 98241771);
        assert_eq!(corporation.creator_id, 379226154);
        assert_eq!(&corporation.date_founded, "2013-08-23T05:45:11Z");
        assert_eq!(&corporation.name, "Fraternity.");
        assert_eq!(&corporation.ticker, "FRT");
        Ok(())
    }
}
