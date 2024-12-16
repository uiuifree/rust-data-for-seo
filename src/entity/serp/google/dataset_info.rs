use crate::entity::{SerpApiElementDataset, SerpApiTaskResult};

pub type SerpApiGoogleDatasetInfo<T> = SerpApiTaskResult<T>;
pub type SerpApiGoogleDatasetInfoAdvanced = SerpApiGoogleDatasetInfo<SerpApiElementDataset>;
