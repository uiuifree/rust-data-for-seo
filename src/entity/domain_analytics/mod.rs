//! Entity models for the domain_analytics API domain.

mod aggregation_technologies;
mod available_technologies;
mod domain_technologies;
mod domains_by_html_terms;
mod domains_by_technology;
mod errors;
mod id_list;
mod technologies_summary;
mod technology_stats;
mod whois_overview;

pub use aggregation_technologies::*;
pub use available_technologies::*;
pub use domain_technologies::*;
pub use domains_by_html_terms::*;
pub use domains_by_technology::*;
pub use errors::*;
pub use id_list::*;
pub use technologies_summary::*;
pub use technology_stats::*;
pub use whois_overview::*;

/// A location supported by the Technologies API.
/// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/locations/>.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct DomainAnalyticsApiTechnologiesLocation {
    /// Full location name.
    pub location_name: Option<String>,
    /// ISO country code.
    pub country_iso_code: Option<String>,
}
