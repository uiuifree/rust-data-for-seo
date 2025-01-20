use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksTimeseriesSummary {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub date: Option<String>,
    pub rank: Option<i32>,
    pub backlinks: Option<i32>,
    pub backlinks_nofollow: Option<i32>,
    pub referring_pages: Option<i32>,
    pub referring_domains: Option<i32>,
    pub referring_domains_nofollow: Option<i32>,
    pub referring_main_domains: Option<i32>,
    pub referring_main_domains_nofollow: Option<i32>,
    pub referring_ips: Option<i32>,
    pub referring_subnets: Option<i32>,
    pub referring_pages_nofollow: Option<i32>,
}
