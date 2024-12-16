use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleLanguage {
    pub language_name: String,
    pub language_code: String,
}
