mod organic;
mod video_comments;
mod video_info;
mod video_subtitles;

pub use organic::*;
pub use video_comments::*;
pub use video_info::*;
pub use video_subtitles::*;

use crate::api::serp::SerpApi;
use crate::entity::SerpApiLanguage;
use crate::{DataForSeoApiResponse, DataForSeoClient, SerpApiLocation};

impl SerpApi<'_> {
    /// Returns the YouTube SERP sub-API.
    /// See <https://docs.dataforseo.com/v3/serp/youtube/overview/>.
    pub fn youtube(&self) -> SerpApiYoutube<'_> {
        SerpApiYoutube {
            client: self.client,
        }
    }
}

/// Youtube SERP data model.
pub struct SerpApiYoutube<'a> {
    pub(crate) client: &'a DataForSeoClient,
}

impl SerpApiYoutube<'_> {
    /// <https://docs.dataforseo.com/v3/serp/youtube/locations/>
    pub async fn locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client.http_get("/v3/serp/youtube/locations").await
    }
    /// <https://docs.dataforseo.com/v3/serp/youtube/languages/>
    pub async fn languages(&self) -> DataForSeoApiResponse<SerpApiLanguage> {
        self.client.http_get("/v3/serp/youtube/languages").await
    }
}
