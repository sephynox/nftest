use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::reward::RewardNFT;
use crate::rewards::Reward;
use crate::services::ErrorResponse;

#[derive(Serialize, Deserialize)]
pub struct RedeemResult {
    pub id: Uuid,
    pub reward: String,
}

#[axum::debug_handler]
pub async fn redeem(Path(id): Path<Uuid>) -> Result<Json<RedeemResult>, ErrorResponse> {
    // Get the user from the repository
    let mut reward: RewardNFT = RewardNFT::from_id(id.to_string()).await?;
    // Get the user's balance of rewards
    let reward = reward.redeem().await?.to_string();

    Ok(Json(RedeemResult { id, reward }))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::services::user::{register, reward, RewardResult};

    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_redeem() {
        // Register a new test user
        let user_id = Uuid::new_v4();
        let result = register(Json(serde_json::json!({ "id": user_id }))).await;

        // Check that the function returned Ok
        assert!(result.is_ok());

        // Reward the user
        let value = 100;
        let result: Result<Json<RewardResult>, ErrorResponse> =
            reward(Path(user_id), Json(serde_json::json!({ "value": value }))).await;
        let reward_id = Uuid::from_str(&result.unwrap().id).unwrap();

        // Call the redeem function
        let result = redeem(Path(reward_id)).await;

        // Check that the function returned Ok
        assert!(result.is_ok());

        // Check that the returned RedeemResult is correct
        let redeem_result = result.unwrap();

        assert_eq!(redeem_result.id, reward_id);
        assert_eq!(redeem_result.reward, value.to_string());
    }
}
