use crate::entity::{SerpApiElementDataset, SerpApiTaskResult};

/// Google Dataset Search result. Items are `dataset` elements.
/// See <https://docs.dataforseo.com/v3/serp/google/dataset_search/task_get/advanced/>.
pub type SerpApiGoogleDatasetSearch<T> = SerpApiTaskResult<T>;
/// Google Dataset Search Advanced result type.
pub type SerpApiGoogleDatasetSearchAdvanced = SerpApiGoogleDatasetSearch<SerpApiElementDataset>;
