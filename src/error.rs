//! Error types returned by the client.

use serde_json::Value;
use std::fmt;

/// Any error produced by a DataForSEO API call.
#[derive(Debug, Clone)]
pub enum DataForSeoError {
    /// The API answered with an error envelope (HTTP 4xx/5xx and a
    /// DataForSEO `status_code` / `status_message`).
    Api(DataForSeoApiError),
    /// The server answered with a non-JSON HTTP failure.
    Http(DataForSeoHttpError),
    /// Transport or (de)serialization failure.
    System(DataForSeoSystemError),
}

impl DataForSeoError {
    /// HTTP status of the failed call, or `0` for transport-level failures.
    pub fn http_status(&self) -> u16 {
        match self {
            DataForSeoError::Api(e) => e.http_status,
            DataForSeoError::Http(e) => e.http_status,
            DataForSeoError::System(_) => 0,
        }
    }

    /// DataForSEO status code (e.g. `40101`) when the API returned one.
    /// See <https://docs.dataforseo.com/v3/appendix/errors/>.
    pub fn api_status_code(&self) -> Option<i32> {
        match self {
            DataForSeoError::Api(e) => Some(e.status_code),
            _ => None,
        }
    }
}

impl fmt::Display for DataForSeoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataForSeoError::Api(e) => write!(
                f,
                "DataForSEO API error (http {}): {} {}",
                e.http_status, e.status_code, e.status_message
            ),
            DataForSeoError::Http(e) => write!(f, "HTTP error {}", e.http_status),
            DataForSeoError::System(e) => write!(f, "client error: {}", e.message),
        }
    }
}

impl std::error::Error for DataForSeoError {}

impl From<DataForSeoApiError> for DataForSeoError {
    fn from(value: DataForSeoApiError) -> Self {
        DataForSeoError::Api(value)
    }
}

impl From<DataForSeoHttpError> for DataForSeoError {
    fn from(value: DataForSeoHttpError) -> Self {
        DataForSeoError::Http(value)
    }
}

impl From<DataForSeoSystemError> for DataForSeoError {
    fn from(value: DataForSeoSystemError) -> Self {
        DataForSeoError::System(value)
    }
}

/// Error envelope returned by the DataForSEO API.
///
/// ```json
/// {"version":"0.1.x","status_code":40101,"status_message":"Authentication failed. ..."}
/// ```
#[derive(Debug, Clone)]
pub struct DataForSeoApiError {
    /// HTTP status of the response.
    pub http_status: u16,
    /// DataForSEO status code, e.g. `40101`.
    /// See <https://docs.dataforseo.com/v3/appendix/errors/>.
    pub status_code: i32,
    /// Human-readable message from the API.
    pub status_message: String,
    /// Raw response body, for debugging.
    pub response_body: String,
}

impl DataForSeoApiError {
    pub(crate) fn from_envelope(http_status: u16, value: &Value, body: String) -> Self {
        DataForSeoApiError {
            http_status,
            status_code: value
                .get("status_code")
                .and_then(Value::as_i64)
                .unwrap_or_default() as i32,
            status_message: value
                .get("status_message")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            response_body: body,
        }
    }
}

/// Non-JSON HTTP failure (e.g. a gateway error page).
#[derive(Debug, Clone)]
pub struct DataForSeoHttpError {
    /// HTTP status of the response.
    pub http_status: u16,
    /// Raw response body, for debugging.
    pub response_body: String,
}

impl DataForSeoHttpError {
    /// Creates an HTTP error from a status code and raw body.
    pub fn new(http_status: u16, response_body: String) -> DataForSeoHttpError {
        DataForSeoHttpError {
            http_status,
            response_body,
        }
    }
}

/// Transport or (de)serialization failure.
#[derive(Debug, Clone)]
pub struct DataForSeoSystemError {
    /// Description of what failed.
    pub message: String,
}

impl DataForSeoSystemError {
    /// Creates a system error from a message.
    pub fn new<T: Into<String>>(message: T) -> DataForSeoSystemError {
        DataForSeoSystemError {
            message: message.into(),
        }
    }
}
