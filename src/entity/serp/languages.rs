use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiLanguage {
    pub language_name: String,
    pub language_code: String,
}
