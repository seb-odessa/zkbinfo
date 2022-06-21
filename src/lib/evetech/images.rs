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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct CorporationIcon {
    pub px128x128: String,
    pub px256x256: String,
    pub px64x64: String,
}
impl CorporationIcon {
    pub async fn from(id: i32) -> anyhow::Result<Self> {
        let url = format!("{EVE_TECH_ROOT}/corporations/{id}/icons/?{EVE_TECH_SERVER}");
        info!("{url}");
        reqwest::get(&url)
            .await?
            .json::<Self>()
            .await
            .map_err(|e| anyhow!(e))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct AllianceIcon {
    pub px128x128: String,
    pub px64x64: String,
}
impl AllianceIcon {
    pub async fn from(id: i32) -> anyhow::Result<Self> {
        let url = format!("{EVE_TECH_ROOT}/alliances/{id}/icons/?{EVE_TECH_SERVER}");
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
    async fn character_portrait_from() {
        let maybe_obj = CharacterPortrait::from(2114350216).await;
        assert!(maybe_obj.is_ok());
        let obj = maybe_obj.unwrap();
        assert_eq!(obj.px64x64, String::from("https://images.evetech.net/characters/2114350216/portrait?tenant=tranquility&size=64"));
        assert_eq!(obj.px128x128, String::from("https://images.evetech.net/characters/2114350216/portrait?tenant=tranquility&size=128"));
        assert_eq!(obj.px256x256, String::from("https://images.evetech.net/characters/2114350216/portrait?tenant=tranquility&size=256"));
        assert_eq!(obj.px512x512, String::from("https://images.evetech.net/characters/2114350216/portrait?tenant=tranquility&size=512"));
    }

    #[tokio::test]
    async fn corporation_icon_from() {
        let maybe_obj = CorporationIcon::from(98573194).await;
        assert!(maybe_obj.is_ok());
        let obj = maybe_obj.unwrap();
        assert_eq!(obj.px64x64, String::from("https://images.evetech.net/corporations/98573194/logo?tenant=tranquility&size=64"));
        assert_eq!(obj.px128x128, String::from("https://images.evetech.net/corporations/98573194/logo?tenant=tranquility&size=128"));
        assert_eq!(obj.px256x256, String::from("https://images.evetech.net/corporations/98573194/logo?tenant=tranquility&size=256"));
    }

    #[tokio::test]
    async fn alliance_icon_from() {
        let maybe_obj = AllianceIcon::from(99011258).await;
        assert!(maybe_obj.is_ok());
        let obj = maybe_obj.unwrap();
        assert_eq!(obj.px64x64, String::from("https://images.evetech.net/Alliance/99011258_64.png"));
        assert_eq!(obj.px128x128, String::from("https://images.evetech.net/Alliance/99011258_128.png"));
    }
}