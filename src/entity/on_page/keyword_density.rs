use serde::{Deserialize, Serialize};
use crate::entity::OnPageDataApiCrawlStatus;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiKeywordDensity {
    pub crawl_progress: Option<String>,
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    pub items_count: Option<i32>,
    pub items: Option<OnPageDataApiKeywordDensityItem>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiKeywordDensityItem {
    pub keyword: Option<String>,
    pub frequency: Option<i32>,
    pub density: Option<i32>,
}

