use crate::DataForSeoClient;

mod amazon;
mod google;

pub use amazon::*;
pub use google::*;

/// Entry point for the Merchant API domain.
/// See <https://docs.dataforseo.com/v3/merchant/>.
pub struct MerchantApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Access the Merchant API (Amazon and Google Shopping).
    /// See <https://docs.dataforseo.com/v3/merchant/>.
    pub fn merchant(&self) -> MerchantApi<'_> {
        MerchantApi { client: self }
    }
}

impl MerchantApi<'_> {
    /// Lists the Merchant tasks created within a time range.
    /// See <https://docs.dataforseo.com/v3/merchant/id_list/>.
    pub async fn id_list(
        &self,
        data: Vec<crate::entity::DataForSeoIdListRequest>,
    ) -> crate::DataForSeoApiResponse<crate::entity::DataForSeoIdListItem> {
        self.client.http_post("/v3/merchant/id_list", &data).await
    }
    /// Lists the Merchant tasks that returned an error in the last 7 days.
    /// See <https://docs.dataforseo.com/v3/merchant/errors/>.
    pub async fn errors(
        &self,
        data: Vec<crate::entity::DataForSeoErrorsRequest>,
    ) -> crate::DataForSeoApiResponse<crate::entity::DataForSeoErrorItem> {
        self.client.http_post("/v3/merchant/errors", &data).await
    }
    /// Lists completed Merchant tasks that have not been collected.
    /// See <https://docs.dataforseo.com/v3/merchant/tasks_ready/>.
    pub async fn tasks_ready(
        &self,
    ) -> crate::DataForSeoApiResponse<crate::entity::MerchantApiTaskReady> {
        self.client.http_get("/v3/merchant/tasks_ready").await
    }
}
