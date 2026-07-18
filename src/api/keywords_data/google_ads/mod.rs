use crate::DataForSeoApiResponse;
use crate::DataForSeoClient;
use crate::api::keywords_data::{KeywordsDataApi, KeywordsDataApiLocation};
use crate::entity::{KeywordsDataApiGoogleAdsStatus, KeywordsDataApiLanguage};

mod ad_traffic_by_keywords;
mod keywords_for_keywords;
mod keywords_for_site;
mod search_volume;

pub use ad_traffic_by_keywords::*;
pub use keywords_for_keywords::*;
pub use keywords_for_site::*;
pub use search_volume::*;

impl KeywordsDataApi<'_> {
    /// Google Ads keyword research endpoints.
    /// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/overview/>.
    pub fn google_ads(&self) -> KeywordsDataApiGoogle<'_> {
        KeywordsDataApiGoogle {
            client: self.client,
        }
    }
}

/// Google Ads keyword research endpoints.
pub struct KeywordsDataApiGoogle<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl KeywordsDataApiGoogle<'_> {
    /// Freshness status of Google Ads keyword data.
    /// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/status/>.
    pub async fn status(&self) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsStatus> {
        self.client
            .http_get("/v3/keywords_data/google_ads/status")
            .await
    }
    /// Locations supported by the Google Ads endpoints.
    /// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/locations/>.
    pub async fn locations(&self) -> DataForSeoApiResponse<KeywordsDataApiLocation> {
        self.client
            .http_get("/v3/keywords_data/google_ads/locations")
            .await
    }
    /// Languages supported by the Google Ads endpoints.
    /// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/languages/>.
    pub async fn languages(&self) -> DataForSeoApiResponse<KeywordsDataApiLanguage> {
        self.client
            .http_get("/v3/keywords_data/google_ads/languages")
            .await
    }
}
