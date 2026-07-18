mod ads_advertisers;
mod ads_search;
mod ai_mode;
mod auto_complete;
mod dataset_info;
mod dataset_search;
mod event;
mod finance;
mod images;
mod jobs;
mod local_finder;
mod maps;
mod news;
mod organic;
mod search_by_image;

pub use ads_advertisers::*;
pub use ads_search::*;
pub use ai_mode::*;
pub use auto_complete::*;
pub use dataset_info::*;
pub use dataset_search::*;
pub use event::*;
pub use finance::*;
pub use images::*;
pub use jobs::*;
pub use local_finder::*;
pub use maps::*;
pub use news::*;
pub use organic::*;
pub use search_by_image::*;
use serde::{Deserialize, Serialize};

/// Google Organic Task Spell SERP data model.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleOrganicTaskSpell {
    /// Search term the result was returned for.
    pub keyword: Option<String>,
    /// Autocorrection type.
    #[serde(rename = "type")]
    pub autocorrection_type: Option<String>,
}
