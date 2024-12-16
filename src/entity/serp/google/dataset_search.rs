use crate::entity::{SerpApiElementDatasetDescription, SerpApiTaskResult};

pub type SerpApiGoogleDatasetSearch<T> = SerpApiTaskResult<T>;
pub type SerpApiGoogleDatasetSearchAdvanced =
    SerpApiGoogleDatasetSearch<SerpApiElementDatasetDescription>;
