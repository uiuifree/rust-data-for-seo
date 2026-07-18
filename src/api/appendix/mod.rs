use crate::entity::AppendixApiUserData;
use crate::{DataForSeoApiResponse, DataForSeoClient};

/// Appendix API: account-level information.
/// See <https://docs.dataforseo.com/v3/appendix/user_data/>.
pub struct AppendixApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Appendix API (account balance, rate limits, pricing).
    pub fn appendix(&self) -> AppendixApi<'_> {
        AppendixApi { client: self }
    }
}

impl AppendixApi<'_> {
    /// Account information: balance, rate limits, usage statistics, and pricing.
    /// Free of charge. See <https://docs.dataforseo.com/v3/appendix/user_data/>.
    pub async fn user_data(&self) -> DataForSeoApiResponse<AppendixApiUserData> {
        self.client.http_get("/v3/appendix/user_data").await
    }
}
