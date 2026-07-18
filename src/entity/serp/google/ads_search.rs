use crate::entity::{SerpApiElementAdsSearch, SerpApiSearchResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Ads Search advanced result.
/// See <https://docs.dataforseo.com/v3/serp/google/ads_search/task_get/advanced/>.
pub type SerpApiGoogleAdsSearchTaskAdvanced = SerpApiSearchResult<SerpApiGoogleAdsSearchItem>;

/// A single item in a Google Ads Search result, tagged by the `type` field.
/// Unrecognized `type` values fall back to [`SerpApiGoogleAdsSearchItem::Unknown`].
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SerpApiGoogleAdsSearchItem {
    /// Element of type `ads_search`.
    #[serde(rename = "ads_search")]
    AdsSearch(Box<SerpApiElementAdsSearch>),
    /// Fallback holding the raw JSON of an unrecognized `type`.
    #[serde(untagged)]
    Unknown(Value),
}
