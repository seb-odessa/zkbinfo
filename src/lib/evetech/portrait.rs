use super::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct CharacterPortrait {
    pub px128x128: String,
    pub px256x256: String,
    pub px512x512: String,
    pub px64x64: String,
}
impl CharacterPortrait {
    pub async fn from(id: i32) -> anyhow::Result<Self> {
        let url = format!("{EVE_TECH_ROOT}/characters/{id}/portrait/?{EVE_TECH_SERVER}");
        info!("{url}");
        reqwest::get(&url)
            .await?
            .json::<Self>()
            .await
            .map_err(|e| anyhow!(e))
    }
}