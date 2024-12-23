use serde::{Deserialize, Serialize};
use crate::entity::OnPageDataApiCrawlStatus;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiWaterfall {
    pub crawl_progress: Option<String>,
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    pub items_count: Option<i32>,
    pub items: Option<Vec<OnPageDataApiWaterfallItem>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiWaterfallItem {
    pub page_url: Option<String>,
    pub time_to_interactive: Option<i32>,
    pub dom_complete: Option<i32>,
    pub connection_time: Option<i32>,
    pub time_to_secure_connection: Option<i32>,
    pub request_sent_time: Option<i32>,
    pub waiting_time: Option<i32>,
    pub download_time: Option<i32>,
    pub duration_time: Option<i32>,
    pub fetch_start: Option<i32>,
    pub fetch_end: Option<i32>,
    pub resources: Option<Vec<OnPageDataApiWaterfallItemResource>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiWaterfallItemResource {
    pub resource_type: Option<String>,
    pub url: Option<String>,
    pub initiator: Option<String>,
    pub duration_time: Option<i32>,
    pub fetch_start: Option<i32>,
    pub fetch_end: Option<i32>,
    pub location: Option<OnPageDataApiWaterfallItemResourceLocation>,
    pub is_render_blocking: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiWaterfallItemResourceLocation {
    pub line: Option<i32>,
    pub offset_left: Option<i32>,
    pub offset_top: Option<i32>,
}

