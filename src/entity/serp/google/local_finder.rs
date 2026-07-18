use crate::entity::{SerpApiElementLocalPack, SerpApiHtmlItem, SerpApiSearchResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Local Finder advanced result.
/// See <https://docs.dataforseo.com/v3/serp/google/local_finder/task_get/advanced/>.
pub type SerpApiGoogleLocalFinderTaskAdvanced = SerpApiSearchResult<SerpApiGoogleLocalFinderItem>;
/// Google Local Finder raw-HTML result.
/// See <https://docs.dataforseo.com/v3/serp/google/local_finder/task_get/html/>.
pub type SerpApiGoogleLocalFinderTaskHtml = SerpApiSearchResult<SerpApiHtmlItem>;

/// A single item in a Google Local Finder SERP, tagged by the DataForSEO `type` field.
/// Unrecognized `type` values fall back to [`SerpApiGoogleLocalFinderItem::Unknown`].
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SerpApiGoogleLocalFinderItem {
    /// Element of type `local_pack`.
    #[serde(rename = "local_pack")]
    LocalPack(Box<SerpApiElementLocalPack>),
    /// Fallback holding the raw JSON of an unrecognized `type`.
    #[serde(untagged)]
    Unknown(Value),
}
