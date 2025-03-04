use crate::entity::BacklinksElementBacklink;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBacklinks {
    pub target: Option<String>,
    pub mode: Option<String>,
    pub custom_mode: Option<String>,
    pub total_count: Option<i32>,
    pub items_count: Option<i32>,
    pub items: Option<Vec<BacklinksElementBacklink>>,
}
