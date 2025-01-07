use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiIndex {
    pub total_backlinks: Option<u64>,
    pub total_pages: Option<u64>,
    pub total_domains: Option<u64>,
    pub index_history: Option<Vec<BacklinksApiIndexHistory>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiIndexHistory {
    pub date: Option<String>,
    pub total_pages: Option<u64>,
    pub total_domains: Option<u64>,
    pub index_history: Option<Vec<BacklinksApiIndexHistory>>,
}
