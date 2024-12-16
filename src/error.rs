use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum DataForSeoError {
    ApiError(DataForSeoApiError),
    HttpError(DataForSeoHttpError),
    SystemError(DataForSeoSystemError),
}

impl DataForSeoError {
    pub fn status(&self) -> u16 {
        match self {
            DataForSeoError::ApiError(e) => e.status,
            DataForSeoError::HttpError(e) => e.status,
            DataForSeoError::SystemError(_) => 0,
        }
    }
    pub fn api_error(&self) -> Option<&DataForSeoApiErrorResponse> {
        match self {
            DataForSeoError::ApiError(e) => Some(&e.error),
            DataForSeoError::HttpError(_) => None,
            DataForSeoError::SystemError(_) => None,
        }
    }
}

impl From<DataForSeoApiError> for DataForSeoError {
    fn from(value: DataForSeoApiError) -> Self {
        DataForSeoError::ApiError(value)
    }
}

impl From<DataForSeoHttpError> for DataForSeoError {
    fn from(value: DataForSeoHttpError) -> Self {
        DataForSeoError::HttpError(value)
    }
}

impl From<DataForSeoSystemError> for DataForSeoError {
    fn from(value: DataForSeoSystemError) -> Self {
        DataForSeoError::SystemError(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataForSeoApiError {
    pub status: u16,
    pub error: DataForSeoApiErrorResponse,
    pub warnings: Option<Vec<String>>,
    pub http_response_body: Option<String>,
}

/// https://developers.line.biz/ja/reference/messaging-api/#error-responses
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataForSeoApiErrorResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<DataForSeoApiErrorResponseDetail>>,
}

/// https://developers.line.biz/ja/reference/messaging-api/#error-responses
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataForSeoApiErrorResponseDetail {
    pub message: String,
    pub property: String,
}

impl DataForSeoApiErrorResponse {
    pub fn debug_print(&self) {
        println!("{}", self.message);
        if let Some(details) = &self.details {
            for detail in details {
                println!(" {}: {}", detail.property, detail.message);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataForSeoHttpError {
    pub status: u16,
    pub http_response_body: Option<String>,
}

impl DataForSeoHttpError {
    pub fn new(status: u16, http_response_body: String) -> DataForSeoHttpError {
        DataForSeoHttpError {
            status,
            http_response_body: Some(http_response_body),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataForSeoSystemError {
    pub message: Option<String>,
}

impl DataForSeoSystemError {
    pub fn new(message: String) -> DataForSeoSystemError {
        DataForSeoSystemError {
            message: Some(message),
        }
    }
}
