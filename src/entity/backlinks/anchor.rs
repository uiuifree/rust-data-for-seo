use crate::entity::BacklinksElementBacklinkAnchor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiAnchor {
    pub target: Option<String>,
    pub total_count: Option<i32>,
    pub items_count: Option<i32>,
    pub items: Option<Vec<BacklinksElementBacklinkAnchor>>,
}
