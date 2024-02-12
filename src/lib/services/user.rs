use axum::extract::Path;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::reward::RewardNFT;
use crate::rewards::Reward;
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
        user.save().await?;

        Ok(Json(RegisterResult { success: true }))
    } else {
        Err(ErrorResponse::from(String::from("Invalid payload")))
    }
}

#[derive(Serialize)]
pub struct BalanceResult {
    pub balance: String,
}

#[axum::debug_handler]
pub async fn get_balance(Path(id): Path<Uuid>) -> Result<Json<BalanceResult>, ErrorResponse> {
    // Get the user from the repository
    let user = User::from_id(id.to_string()).await?;
    // Get the user's balance of rewards
    let balance = user.get_reward_balance().await;

    Ok(Json(BalanceResult { balance }))
}

#[derive(Deserialize)]
struct RewardRequest {
    pub value: u128,
}

#[derive(Serialize)]
pub struct RewardResult {
    pub success: bool,
    pub id: String,
    pub url: String,
}

#[axum::debug_handler]
pub async fn reward(
    Path(id): Path<Uuid>,
    payload: Json<serde_json::Value>,
) -> Result<Json<RewardResult>, ErrorResponse> {
    // TODO validate payload
    let request: Result<RewardRequest, serde_json::Error> = serde_json::from_value(payload.0);

    if let Ok(data) = request {
        let value = data.value;
        // Get the user from the repository
        let user = User::from_id(id.to_string()).await?;
        // Reward the user
        let reward = RewardNFT::new(value);
        let user_wallet = user.get_wallet()?;

        // Mint the reward
        reward.mint(&user_wallet).await?;

        Ok(Json(RewardResult {
            success: true,
            id: reward.get_id(),
            url: reward.get_url(),
        }))
    } else {
        Err(ErrorResponse::from(String::from("Invalid payload")))
    }
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

    #[tokio::test]
    async fn test_get_balance_success() {
        // Generate a unique UUID for this test
        let id = Uuid::new_v4();

        // Register the user
        let result = register_user(id.to_string().clone()).await;

        // Check the result
        assert!(result.is_ok());

        // Get the user's balance
        let result = get_balance(Path(id)).await;

        // Check the result
        assert!(result.is_ok());
        assert!(!result.unwrap().0.balance.is_empty());
    }

    #[tokio::test]
    async fn test_reward_success() {
        // Generate a unique UUID for this test
        let id = Uuid::new_v4();

        // Register the user
        let result = register_user(id.to_string().clone()).await;

        // Check the result
        assert!(result.is_ok());

        // Reward the user
        let result = reward(Path(id), Json(json!({ "value": 100 }))).await;

        // Check the result
        assert!(result.is_ok());

        let result = result.unwrap();

        assert!(result.0.success);
        assert!(!result.0.id.is_empty());
        assert!(!result.0.url.is_empty());
    }
}
