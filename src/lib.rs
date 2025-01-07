mod api;
pub mod entity;
pub mod error;
pub use api::*;

use crate::entity::DataForSeoApiResponseData;
use crate::error::{
    DataForSeoApiError, DataForSeoError, DataForSeoHttpError, DataForSeoSystemError,
};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use reqwest::{Body, Method, RequestBuilder, Url};
use serde_json::{json, Value};

pub struct DataForSeoClient {
    token: String,
}



impl DataForSeoClient {
    pub fn new<T: Into<String>>(id: T, password: T) -> DataForSeoClient {
        DataForSeoClient {
            token: BASE64_STANDARD.encode(id.into() + ":" + password.into().as_str()),
        }
    }
}

pub type DataForSeoApiResponse<R> = Result<DataForSeoApiResponseData<R>, DataForSeoError>;

impl DataForSeoClient {
    async fn http_response_reqwest<R>(
        response: Result<reqwest::Response, reqwest::Error>,
    ) -> DataForSeoApiResponse<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        let response = match response {
            Ok(v) => v,
            Err(e) => return Err(DataForSeoSystemError::new(e.to_string()).into()),
        };

        let status = response.status();

        let body = match response.bytes().await {
            Ok(v) => v.to_vec(),
            Err(e) => {
                return Err(DataForSeoHttpError::new(status.as_u16(), e.to_string()).into());
            }
        };

        let http_response_body = match String::from_utf8(body) {
            Ok(v) => v,
            Err(e) => return Err(DataForSeoSystemError::new(e.to_string()).into()),
        };

        let value = match serde_json::from_str::<Value>(http_response_body.as_str()) {
            Ok(v) => v,
            Err(e) => return Err(DataForSeoSystemError::new(e.to_string()).into()),
        };

        Self::http_response_text(status.as_u16(), http_response_body, value).await
    }
    async fn http_response_text<R>(
        status: u16,
        http_response_body: String,
        value: Value,
    ) -> DataForSeoApiResponse<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        if 400 <= status {
            if let Ok(res) = serde_json::from_value(value.clone()) {
                return Err(DataForSeoApiError {
                    status,
                    error: res,
                    warnings: None,
                    http_response_body: Some(http_response_body),
                }
                .into());
            }
        }

        match serde_json::from_value(value.clone()) {
            Ok(v) => Ok(v),
            Err(e) => {
                if let Ok(res) = serde_json::from_value(value.clone()) {
                    return Err(DataForSeoApiError {
                        status,
                        error: res,
                        warnings: None,
                        http_response_body: Some(http_response_body),
                    }
                    .into());
                }
                Err(DataForSeoSystemError::new(e.to_string()).into())
            }
        }
    }
    pub(crate) async fn http_get<P, R>(&self, url: &str, value: &P) -> DataForSeoApiResponse<R>
    where
        P: serde::Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        let build = builder2(Url::parse(url).unwrap(), Method::GET, self.token.as_str());
        let request = build.body(Body::from(json!(value).to_string()));
        DataForSeoClient::http_response_reqwest(request.send().await).await
    }
    // pub(crate) async fn http_get_stream<P>(&self, url: &str, value: &P) -> DataForSeoApiResponse<Vec<u8>>
    // where
    //     P: serde::Serialize,
    // {
    //     let build = builder2(
    //         Url::parse(url).unwrap(),
    //         Method::GET,
    //         self.token.as_str(),
    //     );
    //     let request = build.body(Body::from(json!(value).to_string()));
    //
    //     let a = request.send().await.unwrap();
    //     let stream = a.bytes().await.unwrap();
    //     Ok(stream.to_vec())
    // }
    pub(crate) async fn http_post<P, R>(&self, url: &str, value: &P) -> DataForSeoApiResponse<R>
    where
        P: serde::Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        let build = builder2(Url::parse(url).unwrap(), Method::POST, self.token.as_str());

        let request = build.json(&json!(value));
        DataForSeoClient::http_response_reqwest(request.send().await).await
    }
    // pub(crate) async fn http_put<P, R>(&self, url: &str, value: &P) -> DataForSeoApiResponse<R>
    // where
    //     P: serde::Serialize,
    //     R: for<'de> serde::Deserialize<'de>,
    // {
    //     let build = builder2(Url::parse(url).unwrap(), Method::PUT, self.token.as_str());
    //
    //     let request = build.json(&json!(value));
    //     DataForSeoClient::http_response_reqwest(request.send().await).await
    // }
    // pub(crate) async fn http_delete<P, R>(&self, url: &str, value: &P) -> DataForSeoApiResponse<R>
    // where
    //     P: serde::Serialize,
    //     R: for<'de> serde::Deserialize<'de>,
    // {
    //     let build = builder2(
    //         Url::parse(url).unwrap(),
    //         Method::DELETE,
    //         self.token.as_str(),
    //     );
    //
    //     let request = build.json(&json!(value));
    //     DataForSeoClient::http_response_reqwest(request.send().await).await
    // }
    // pub(crate) async fn http_post_data<R>(&self, url: &str, content: Vec<u8>) -> DataForSeoApiResponse<R>
    // where
    //     R: for<'de> serde::Deserialize<'de>,
    // {
    //     let req = reqwest::Client::new()
    //         .post(url)
    //         .header(
    //             "Authorization",
    //             format!("Bearer {}", self.token.to_string()),
    //         )
    //         .header("Content-Type", "image/jpeg")
    //         .body(content)
    //         .send()
    //         .await;
    //
    //     Self::http_response_reqwest(req).await
    // }
}

fn builder2(url: Url, method: reqwest::Method, token: &str) -> RequestBuilder {
    reqwest::Client::new()
        .request(method, url)
        .header("Authorization", format!("Basic {}", token))
}
