mod bing;
mod clickstream_data;
mod dataforseo_trends;
mod google_ads;
mod google_trends;

pub use bing::*;
pub use clickstream_data::*;
pub use dataforseo_trends::*;
pub use google_ads::*;
pub use google_trends::*;

use serde::{Deserialize, Serialize};

/// A language supported by a Keywords Data sub-API.
/// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/languages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiLanguage {
    /// Full language name, e.g. "English".
    pub language_name: Option<String>,
    /// ISO 639-1 language code, e.g. "en".
    pub language_code: Option<String>,
}
