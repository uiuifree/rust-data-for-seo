mod backlinks;
mod element;
mod index;
mod summary;
mod anchor;

use serde::{Deserialize, Serialize};
pub use backlinks::*;
pub use element::*;
pub use index::*;
pub use summary::*;
pub use anchor::*;


#[derive(Debug, Default, Serialize, Deserialize, Clone)]

pub struct BacklinksIndirectLinkPath {
    pub page_from_keywords_count_top_3: Option<i32>,
    pub page_from_keywords_count_top_10: Option<i32>,
    pub page_from_keywords_count_top_100: Option<i32>,
}
