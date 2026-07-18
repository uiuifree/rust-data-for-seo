use crate::entity::{SerpApiElementMapsSearch, SerpApiHtmlItem, SerpApiSearchResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Maps advanced result.
/// See <https://docs.dataforseo.com/v3/serp/google/maps/task_get/advanced/>.
pub type SerpApiGoogleMapsTaskAdvanced = SerpApiSearchResult<SerpApiGoogleMapsItem>;
/// Google Maps raw-HTML result.
/// See <https://docs.dataforseo.com/v3/serp/google/maps/task_get/html/>.
pub type SerpApiGoogleMapsTaskHtml = SerpApiSearchResult<SerpApiHtmlItem>;

/// A single item in a Google Maps SERP, tagged by the DataForSEO `type` field.
/// Unrecognized `type` values fall back to [`SerpApiGoogleMapsItem::Unknown`].
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SerpApiGoogleMapsItem {
    /// Element of type `maps_search`.
    #[serde(rename = "maps_search")]
    MapsSearch(Box<SerpApiElementMapsSearch>),
    /// Fallback holding the raw JSON of an unrecognized `type`.
    #[serde(untagged)]
    Unknown(Value),
}
