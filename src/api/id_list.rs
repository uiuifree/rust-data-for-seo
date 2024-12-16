use crate::entity::SerpApiIdListRequest;
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde_json::Value;

impl DataForSeoClient {
    /// https://developers.line.biz/ja/reference/messaging-api/#get-bot-info
    pub async fn bot_info(&self, data: Vec<SerpApiIdListRequest>) -> DataForSeoApiResponse<Value> {
        self.http_post("https://api.dataforseo.com/v3/serp/id_list", &data)
            .await
    }
}
