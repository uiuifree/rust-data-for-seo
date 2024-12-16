use crate::{DataForSeoClient};
use crate::api::keywords_data::KeywordsDataApi;

mod search_volume;
mod keywords_for_site;
mod keywords_for_keywords;
mod ad_traffic_by_keywords;

pub use search_volume::*;
pub use keywords_for_site::*;
pub use keywords_for_keywords::*;
pub use ad_traffic_by_keywords::*;



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
