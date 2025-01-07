use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksElementBacklinkAnchor {
    pub backlinks:Option<i32>,
    pub first_seen: Option<String>,
    pub lost_date: Option<String>,
    pub backlinks_spam_score: Option<i32>,
    pub broken_backlinks: Option<i32>,
    pub broken_pages: Option<i32>,
    pub referring_domains: Option<i32>,
    pub referring_domains_nofollow: Option<i32>,
    pub referring_main_domains: Option<i32>,
    pub referring_main_domains_nofollow: Option<i32>,
    pub referring_ips: Option<i32>,
    pub referring_subnets: Option<i32>,
    pub referring_pages: Option<i32>,
    pub referring_links_tld: Option<HashMap<String, i32>>,
    pub referring_links_types: Option<HashMap<String, i32>>,
    pub referring_links_attributes: Option<HashMap<String, i32>>,
    pub referring_links_platform_types: Option<HashMap<String, i32>>,
    pub referring_links_semantic_locations: Option<HashMap<String, i32>>,
    pub referring_links_countries: Option<HashMap<String, i32>>,
    pub referring_pages_nofollow: Option<i32>,

}