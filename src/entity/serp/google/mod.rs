mod auto_complete;
mod dataset_info;
mod dataset_search;
mod event;
mod images;
mod jobs;
mod news;
mod organic;

pub use auto_complete::*;
pub use dataset_info::*;
pub use dataset_search::*;
pub use event::*;
pub use images::*;
pub use jobs::*;
pub use news::*;
pub use organic::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleOrganicTaskSpell {
    pub keyword: Option<String>,
    #[serde(rename = "type")]
    pub autocorrection_type: Option<String>,
}
