//! Social Media endpoints of the Business Data API.
//! See <https://docs.dataforseo.com/v3/business_data/social_media/overview/>.

use crate::api::business_data::BusinessDataApi;
use crate::entity::{BusinessDataApiPinterestItem, BusinessDataApiSocialMediaLiveRequest};
use crate::{DataForSeoApiResponse, DataForSeoClient};

impl BusinessDataApi<'_> {
    /// Returns the Social Media sub-API of the Business Data domain.
    /// See <https://docs.dataforseo.com/v3/business_data/social_media/overview/>.
    pub fn social_media(&self) -> BusinessDataApiSocialMedia<'_> {
        BusinessDataApiSocialMedia {
            client: self.client,
        }
    }
}

/// Social Media endpoints: Pinterest engagement counts.
pub struct BusinessDataApiSocialMedia<'a> {
    client: &'a DataForSeoClient,
}

impl BusinessDataApiSocialMedia<'_> {
    /// Returns the number of Pinterest pins for each target URL, synchronously.
    /// See <https://docs.dataforseo.com/v3/business_data/social_media/pinterest/live/>.
    pub async fn pinterest_live(
        &self,
        data: Vec<BusinessDataApiSocialMediaLiveRequest>,
    ) -> DataForSeoApiResponse<BusinessDataApiPinterestItem> {
        self.client
            .http_post("/v3/business_data/social_media/pinterest/live", &data)
            .await
    }
}
