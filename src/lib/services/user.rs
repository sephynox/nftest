use axum::Json;
use serde::{Deserialize, Serialize};

use crate::models::user::UserError;
use crate::{core::chain::generate_secret_key, models::user::User};

use super::ErrorResponse;

#[derive(Deserialize)]
struct RegisterRequest {
    pub id: String,
}

#[derive(Serialize)]
pub struct RegisterResult {
    pub success: bool,
}

#[axum::debug_handler]
pub async fn register(
    payload: Json<serde_json::Value>,
) -> Result<Json<RegisterResult>, ErrorResponse> {
    // TODO validate payload
    let request: Result<RegisterRequest, serde_json::Error> = serde_json::from_value(payload.0);

    if let Ok(data) = request {
        let id = data.id;
        // Generate a new key
        let key = generate_secret_key();
        // Create a new user
        let user = User::new(id, key);

        // Save the user to the repository
        user.save().await.map_err(|error| match error {
            UserError::AlreadyExists => ErrorResponse::from(String::from("User already exists")),
            _ => ErrorResponse::from(String::from("Failed to save user")),
        })?;

        Ok(Json(RegisterResult { success: true }))
    } else {
        Err(ErrorResponse::from(String::from("Invalid payload")))
    }
}

#[derive(Deserialize)]
pub struct BalanceRequest {
    pub id: String,
}

#[derive(Serialize)]
pub struct BalanceResult {
    pub balance: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{http::StatusCode, Json};
    use serde_json::json;
    use uuid::Uuid;

    async fn register_user(id: String) -> Result<Json<RegisterResult>, ErrorResponse> {
        // Create a mock payload
        let payload = Json(json!({
            "id": id
        }));

        // Call the register function
        register(payload).await
    }

    #[tokio::test]
    async fn test_register_success() {
        // Generate a unique UUID for this test
        let id = Uuid::new_v4().to_string();

        // Register the user
        let result = register_user(id).await;

        // Check the result
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_existing_user() {
        // Generate a unique UUID for this test
        let id = Uuid::new_v4().to_string();

        // Register the user
        let result = register_user(id.clone()).await;

        // Check the result
        assert!(result.is_ok());

        // Try to register the user again
        let result = register_user(id).await;

        // Check the result
        match result {
            Ok(_) => {
                panic!("Should have failed to register user with existing id");
            }
            Err(error) => {
                assert_eq!(error.status, StatusCode::BAD_REQUEST);
            }
        }
    }
}
