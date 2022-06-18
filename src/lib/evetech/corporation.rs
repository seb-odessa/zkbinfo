use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Corporation {
    pub alliance_id: Option<i32>,
    pub ceo_id: i32,
    pub creator_id: i32,
    pub date_founded: Option<String>,
    pub description: Option<String>,
    pub faction_id: Option<i32>,
    pub home_station_id: Option<i32>,
    pub member_count: i32,
    pub name: String,
    pub shares: Option<i32>,
    pub tax_rate: f32,
    pub ticker: String,
    pub url: Option<String>,
    pub war_eligible: Option<bool>,
}
impl Corporation {
    pub async fn from(id: i32) -> anyhow::Result<Self> {
        let url = format!("{EVE_TECH_ROOT}/corporations/{id}/?{EVE_TECH_SERVER}");
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
        let corporation = Corporation::from(98573194).await.map_err(|e| format!("{e}"))?;

        assert_eq!(corporation.ceo_id, 2115657646);
        assert_eq!(corporation.creator_id, 2114350216);
        assert_eq!(corporation.date_founded, Some(String::from("2018-09-05T18:41:42Z")));
        assert_eq!(&corporation.name, "SO Corporation");
        assert_eq!(&corporation.ticker, "SO C");
        Ok(())
    }
}
