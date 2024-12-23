use serde::{Deserialize, Serialize};
use crate::entity::OnPageDataApiCrawlStatus;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiRawHtml {
    pub crawl_progress: Option<String>,
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    pub items_count: Option<i32>,
    pub items: Option<OnPageDataApiRawHtmlItem>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiRawHtmlItem {
    pub html: Option<String>,
}

