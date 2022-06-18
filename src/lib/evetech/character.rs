use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Character {
    pub corporation_id: i32,
    pub alliance_id: Option<i32>,
    pub faction_id: Option<i32>,
    pub birthday: String,
    pub bloodline_id: i32,
    pub race_id: i32,
    pub name: String,
    pub gender: String,
    pub description: Option<String>,
    pub security_status: f32,
    pub title: Option<String>,
}
impl Character {
    pub async fn from(id: i32) -> anyhow::Result<Self> {
        let url = format!("{EVE_TECH_ROOT}/characters/{id}/?{EVE_TECH_SERVER}");
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
        let character = Character::from(2114350216).await.map_err(|e| format!("{e}"))?;

        assert_eq!(&character.birthday, "2018-07-27T17:42:45Z");
        assert_eq!(&character.gender, "male");
        assert_eq!(&character.name, "Seb Odessa");
        assert_eq!(character.bloodline_id, 1);
        assert_eq!(character.race_id, 1);
        Ok(())
    }
}
