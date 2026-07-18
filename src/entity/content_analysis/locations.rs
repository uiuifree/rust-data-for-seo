use serde::{Deserialize, Serialize};

/// Result item of `content_analysis/locations`: a location the API can filter by.
/// See <https://docs.dataforseo.com/v3/content_analysis/locations/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiLocation {
    /// Full name of the location.
    pub location_name: Option<String>,
    /// ISO country code of the location.
    pub country_iso_code: Option<String>,
}
