use anyhow::anyhow;
use log::info;
use serde::{Deserialize, Serialize};

pub const EVE_TECH_ROOT: &str = "https://esi.evetech.net/latest";
pub const EVE_TECH_SERVER: &str = "datasource=tranquility";
pub const EVE_TECH_SEARCH: &str = "language=en&strict=true";

mod alliance;
mod character;
mod corporation;
mod killmail;
mod images;
mod search;

pub use alliance::Alliance;
pub use character::Character;
pub use corporation::Corporation;
pub use killmail::Killmail;
pub use images::CharacterPortrait;
pub use images::CorporationIcon;
pub use images::AllianceIcon;

pub use search::SearchCategory;
pub use search::SearchResult;
