use crate::entity::{SerpApiElementLinkElement, SerpApiRectangle};
use serde::{Deserialize, Serialize};

/// AnswerBox
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAnswerBox {
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub text: Option<String>,
    pub links: Option<Vec<SerpApiElementLinkElement>>,
    pub rectangle: Option<SerpApiRectangle>,
}
