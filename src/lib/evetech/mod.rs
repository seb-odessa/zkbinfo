use anyhow::anyhow;
use log::info;
use serde::{Deserialize, Serialize};

pub const EVE_TECH_ROOT: &str = "https://esi.evetech.net/latest";
pub const EVE_TECH_SERVER: &str = "datasource=tranquility";
pub const EVE_TECH_SEARCH: &str = "language=en&strict=true";

mod character;
mod killmail;
mod portrait;
mod search;

pub use character::Character;
pub use killmail::Killmail;
pub use portrait::CharacterPortrait;
pub use search::SearchCategory;
pub use search::SearchResult;
