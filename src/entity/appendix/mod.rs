//! Entity models for the Appendix API domain.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Account information returned by the User Data endpoint.
/// See <https://docs.dataforseo.com/v3/appendix/user_data/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppendixApiUserData {
    /// Account login.
    pub login: Option<String>,
    /// Time zone configured for the account.
    pub timezone: Option<String>,
    /// API rate limits and usage statistics.
    pub rates: Option<AppendixApiUserDataRates>,
    /// Deposited funds, remaining balance, and spending statistics.
    pub money: Option<AppendixApiUserDataMoney>,
    /// Pricing per API/function/priority; keys are dynamic, kept as raw JSON.
    pub price: Option<Value>,
    /// Expiry of the Backlinks API subscription, UTC; `None` when inactive.
    pub backlinks_subscription_expiry_date: Option<String>,
    /// Expiry of the LLM Mentions subscription, UTC; `None` when inactive.
    pub llm_mentions_subscription_expiry_date: Option<String>,
}

/// API rate limits and usage statistics.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppendixApiUserDataRates {
    /// Limits per time grouping (`day`, `minute`); keys are dynamic, kept as raw JSON.
    pub limits: Option<Value>,
    /// Usage statistics per time grouping and function; keys are dynamic, kept as raw JSON.
    pub statistics: Option<Value>,
}

/// Deposited funds, remaining balance, and spending statistics.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppendixApiUserDataMoney {
    /// Total funds deposited into the account, USD.
    pub total: Option<f64>,
    /// Remaining account balance, USD.
    pub balance: Option<f64>,
    /// Cost thresholds per time grouping; keys are dynamic, kept as raw JSON.
    pub limits: Option<Value>,
    /// Spending breakdown per function; keys are dynamic, kept as raw JSON.
    pub statistics: Option<Value>,
}
