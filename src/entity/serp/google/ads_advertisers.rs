use crate::entity::{
    SerpApiElementAdsAdvertiser, SerpApiElementAdsDomain, SerpApiElementAdsMultiAccountAdvertiser,
    SerpApiSearchResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Ads Advertisers advanced result.
/// See <https://docs.dataforseo.com/v3/serp/google/ads_advertisers/task_get/advanced/>.
pub type SerpApiGoogleAdsAdvertisersTaskAdvanced =
    SerpApiSearchResult<SerpApiGoogleAdsAdvertisersItem>;

/// A single item in a Google Ads Advertisers result, tagged by the `type` field.
/// Unrecognized `type` values fall back to [`SerpApiGoogleAdsAdvertisersItem::Unknown`].
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SerpApiGoogleAdsAdvertisersItem {
    /// Element of type `ads_advertiser`.
    #[serde(rename = "ads_advertiser")]
    AdsAdvertiser(Box<SerpApiElementAdsAdvertiser>),
    /// Element of type `ads_multi_account_advertiser`.
    #[serde(rename = "ads_multi_account_advertiser")]
    AdsMultiAccountAdvertiser(Box<SerpApiElementAdsMultiAccountAdvertiser>),
    /// Element of type `ads_domain`.
    #[serde(rename = "ads_domain")]
    AdsDomain(Box<SerpApiElementAdsDomain>),
    /// Fallback holding the raw JSON of an unrecognized `type`.
    #[serde(untagged)]
    Unknown(Value),
}
