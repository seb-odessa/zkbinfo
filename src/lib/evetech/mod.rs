use log::info;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

pub const EVE_TECH_ROOT: &str = "https://esi.evetech.net/latest";
pub const EVE_TECH_SERVER: &str = "datasource=tranquility";
pub const EVE_TECH_SEARCH: &str = "language=en&strict=true";

pub mod search;
pub mod portrait;
pub mod killmail;
pub mod character;

pub use search::SearchCategory;
pub use search::SearchResult;
pub use portrait::CharacterPortrait;
pub use killmail::Killmail;
pub use character::Character;


