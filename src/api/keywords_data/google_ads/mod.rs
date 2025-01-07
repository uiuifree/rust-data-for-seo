use crate::api::keywords_data::KeywordsDataApi;
use crate::DataForSeoClient;

mod ad_traffic_by_keywords;
mod keywords_for_keywords;
mod keywords_for_site;
mod search_volume;

pub use ad_traffic_by_keywords::*;
pub use keywords_for_keywords::*;
pub use keywords_for_site::*;
pub use search_volume::*;

impl KeywordsDataApi<'_> {
    /// https://developers.line.biz/ja/reference/messaging-api/#get-bot-info
    pub fn google_ads(&self) -> KeywordsDataApiGoogle<'_> {
        KeywordsDataApiGoogle {
            client: self.client,
        }
    }
}

pub struct KeywordsDataApiGoogle<'a> {
    client: &'a DataForSeoClient,
}
