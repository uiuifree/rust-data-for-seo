use crate::entity::{SerpApiElementDataset, SerpApiTaskResult};

/// Google Dataset Info result type.
pub type SerpApiGoogleDatasetInfo<T> = SerpApiTaskResult<T>;
/// Google Dataset Info Advanced result type.
pub type SerpApiGoogleDatasetInfoAdvanced = SerpApiGoogleDatasetInfo<SerpApiElementDataset>;
