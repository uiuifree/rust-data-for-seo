use crate::entity::{
    BacklinksApiAnchor, BacklinksApiBacklinks, BacklinksApiIndex, BacklinksApiSummary,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};

pub struct BacklinksApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    pub fn backlinks(&self) -> BacklinksApi {
        BacklinksApi { client: self }
    }
}

impl BacklinksApi<'_> {
    pub async fn index(&self) -> DataForSeoApiResponse<BacklinksApiIndex> {
        self.client
            .http_get("https://api.dataforseo.com/v3/backlinks/index", &{})
            .await
    }
    pub async fn summary(
        &self,
        data: Vec<BacklinksApiSummaryPost>,
    ) -> DataForSeoApiResponse<BacklinksApiSummary> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/backlinks/summary/live",
                &data,
            )
            .await
    }
    // pub async fn history(&self,data:Vec<BacklinksApiSummaryPost>) -> DataForSeoApiResponse<Value> {
    //     self.client
    //         .http_post("https://api.dataforseo.com/v3/backlinks/summary/live", &data)
    //         .await
    // }
    pub async fn backlinks(
        &self,
        data: Vec<BacklinksApiBacklinksPost>,
    ) -> DataForSeoApiResponse<BacklinksApiBacklinks> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/backlinks/backlinks/live",
                &data,
            )
            .await
    }
    pub async fn anchor(
        &self,
        data: Vec<BacklinksApiAnchorPost>,
    ) -> DataForSeoApiResponse<BacklinksApiAnchor> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/backlinks/anchors/live",
                &data,
            )
            .await
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiSummaryPost {
    pub target: String,
    pub include_subdomains: Option<bool>,
    pub include_indirect_links: Option<bool>,
    pub exclude_internal_backlinks: Option<bool>,
    pub internal_list_limit: Option<u32>,
    pub backlinks_status_type: Option<String>,
    pub backlinks_filters: Option<Vec<Vec<String>>>,
    pub tag: Option<String>,
}

impl BacklinksApiSummaryPost {
    pub fn new(target: &str) -> BacklinksApiSummaryPost {
        BacklinksApiSummaryPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBacklinksPost {
    pub target: String,
    pub mode: Option<String>,
    pub custom_mode: Option<String>,
    pub filters: Option<Vec<Vec<String>>>,
    pub order_by: Option<Vec<String>>,
    pub offset: Option<u32>,
    pub search_after_token: Option<String>,
    pub limit: Option<u32>,
    pub backlinks_status_type: Option<String>,
    pub include_subdomains: Option<bool>,
    pub include_indirect_links: Option<bool>,
    pub exclude_internal_backlinks: Option<bool>,
    pub tag: Option<String>,
}

impl BacklinksApiBacklinksPost {
    pub fn new(target: &str) -> BacklinksApiBacklinksPost {
        BacklinksApiBacklinksPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiAnchorPost {
    pub target: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub internal_list_limit: Option<u32>,
    pub backlinks_status_type: Option<String>,
    pub filters: Option<Vec<Vec<String>>>,
    pub order_by: Option<Vec<String>>,
    pub backlinks_filters: Option<Vec<Vec<String>>>,
    pub include_subdomains: Option<bool>,
    pub include_indirect_links: Option<bool>,
    pub exclude_internal_backlinks: Option<bool>,
    pub tag: Option<String>,
}

impl BacklinksApiAnchorPost {
    pub fn new(target: &str) -> BacklinksApiAnchorPost {
        BacklinksApiAnchorPost {
            target: target.to_string(),
            ..Default::default()
        }
    }
}
