use axum::{body::Body, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{core::reward::RewardError, models::user::UserError};

pub mod reward;
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

impl From<UserError> for ErrorResponse {
    fn from(error: UserError) -> Self {
        match error {
            UserError::NotFound => ErrorResponse {
                status: StatusCode::NOT_FOUND,
                error: ErrorDetails {
                    kind: "NotFoundError".into(),
                    message: "User not found".into(),
                },
            },
            UserError::AlreadyExists => ErrorResponse {
                status: StatusCode::BAD_REQUEST,
                error: ErrorDetails {
                    kind: "ValidationError".into(),
                    message: "User already exists".into(),
                },
            },
            UserError::RepositoryError(e) => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: ErrorDetails {
                    kind: "RepositoryError".into(),
                    message: format!("{:?}", e),
                },
            },
            UserError::UnknownError(e) => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: ErrorDetails {
                    kind: "UnknownError".into(),
                    message: format!("{:?}", e),
                },
            },
        }
    }
}

impl From<RewardError> for ErrorResponse {
    fn from(error: RewardError) -> Self {
        match error {
            RewardError::NotFound => ErrorResponse {
                status: StatusCode::NOT_FOUND,
                error: ErrorDetails {
                    kind: "NotFoundError".into(),
                    message: "Reward not found".into(),
                },
            },
            RewardError::AlreadyExists => ErrorResponse {
                status: StatusCode::BAD_REQUEST,
                error: ErrorDetails {
                    kind: "ValidationError".into(),
                    message: "Reward already exists".into(),
                },
            },
            RewardError::AlreadyRedeemed => ErrorResponse {
                status: StatusCode::BAD_REQUEST,
                error: ErrorDetails {
                    kind: "ValidationError".into(),
                    message: "Reward already redeemed".into(),
                },
            },
            RewardError::RepositoryError(e) => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: ErrorDetails {
                    kind: "RepositoryError".into(),
                    message: format!("{:?}", e),
                },
            },
            RewardError::UserError(e) => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: ErrorDetails {
                    kind: "UserError".into(),
                    message: format!("{:?}", e),
                },
            },
            RewardError::MintRewardError => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: ErrorDetails {
                    kind: "MintRewardError".into(),
                    message: "Failed to mint reward".into(),
                },
            },
            RewardError::UnknownError(e) => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: ErrorDetails {
                    kind: "UnknownError".into(),
                    message: format!("{:?}", e),
                },
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
