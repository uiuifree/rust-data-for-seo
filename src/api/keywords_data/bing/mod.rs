use crate::DataForSeoApiResponse;
use crate::DataForSeoClient;
use crate::api::keywords_data::{KeywordsDataApi, KeywordsDataApiLocation};
use crate::entity::KeywordsDataApiLanguage;

mod audience_estimation;
mod keyword_performance;
mod keyword_suggestions_for_url;
mod keywords_for_keywords;
mod keywords_for_site;
mod search_volume;
mod search_volume_history;

pub use audience_estimation::*;
pub use keyword_performance::*;
pub use keyword_suggestions_for_url::*;
pub use keywords_for_keywords::*;
pub use keywords_for_site::*;
pub use search_volume::*;
pub use search_volume_history::*;

impl KeywordsDataApi<'_> {
    /// Bing Ads keyword research endpoints.
    /// See <https://docs.dataforseo.com/v3/keywords_data/bing/overview/>.
    pub fn bing(&self) -> KeywordsDataApiBing<'_> {
        KeywordsDataApiBing {
            client: self.client,
        }
    }
}

/// Bing Ads keyword research endpoints.
pub struct KeywordsDataApiBing<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl KeywordsDataApiBing<'_> {
    /// Locations supported by the Bing endpoints.
    /// See <https://docs.dataforseo.com/v3/keywords_data/bing/locations/>.
    pub async fn locations(&self) -> DataForSeoApiResponse<KeywordsDataApiLocation> {
        self.client
            .http_get("/v3/keywords_data/bing/locations")
            .await
    }
    /// Languages supported by the Bing endpoints.
    /// See <https://docs.dataforseo.com/v3/keywords_data/bing/languages/>.
    pub async fn languages(&self) -> DataForSeoApiResponse<KeywordsDataApiLanguage> {
        self.client
            .http_get("/v3/keywords_data/bing/languages")
            .await
    }
}
