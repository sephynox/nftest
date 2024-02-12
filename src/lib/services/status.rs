use axum::Json;
use serde::{Deserialize, Serialize};

/// Status contains the version of the application and the current time.
#[derive(Serialize, Deserialize)]
pub struct Status {
    pub version: String,
}

#[axum::debug_handler]
pub async fn status() -> Json<Status> {
    Json(Status {
        version: crate::VERSION.into(),
    })
}
