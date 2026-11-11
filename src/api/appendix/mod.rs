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
    /// Lists the API error codes and messages.
    /// See <https://docs.dataforseo.com/v3/appendix/errors/>.
    pub async fn errors(&self) -> crate::DataForSeoApiResponse<crate::entity::AppendixApiError> {
        self.client.http_get("/v3/appendix/errors").await
    }
    /// Returns the current status of each API.
    /// See <https://docs.dataforseo.com/v3/appendix/status/>.
    pub async fn status(&self) -> crate::DataForSeoApiResponse<crate::entity::AppendixApiStatus> {
        self.client.http_get("/v3/appendix/status").await
    }
    /// Resends the pingback or postback of completed tasks.
    /// See <https://docs.dataforseo.com/v3/appendix/webhook_resend/>.
    pub async fn webhook_resend(
        &self,
        data: Vec<AppendixApiWebhookResendPost>,
    ) -> crate::DataForSeoApiResponse<serde_json::Value> {
        self.client
            .http_post("/v3/appendix/webhook_resend", &data)
            .await
    }
}

/// Request body for `appendix/webhook_resend`.
/// See <https://docs.dataforseo.com/v3/appendix/webhook_resend/>.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct AppendixApiWebhookResendPost {
    /// Identifier of the task.
    pub id: String,
}

impl AppendixApiWebhookResendPost {
    /// Creates a request for the given task.
    pub fn new<T: Into<String>>(id: T) -> Self {
        AppendixApiWebhookResendPost { id: id.into() }
    }
}
