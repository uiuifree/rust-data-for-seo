use serde::{Deserialize, Serialize};
use crate::entity::{OnPageDataApiCrawlStatus, OnPageDataApiRawHtml, OnPageResourceHtml};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiPage {
    pub crawl_progress: Option<String>,
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    pub total_items_count: Option<i32>,
    pub items_count: Option<i32>,
    pub items: Option<Vec<OnPageResourceHtml>>,
}
