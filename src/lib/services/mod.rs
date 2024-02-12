use axum::{body::Body, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

pub mod status;
pub mod user;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorDetails {
    pub kind: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(skip)]
    pub status: StatusCode,
    pub error: ErrorDetails,
}

impl From<String> for ErrorResponse {
    fn from(message: String) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            error: ErrorDetails {
                kind: "ValidationError".into(),
                message,
            },
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self.error).unwrap();
        axum::response::Response::builder()
            .status(self.status)
            .body(Body::from(body))
            .unwrap()
    }
}
