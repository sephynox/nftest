use axum::{extract::Path, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::models::reward::RewardNFT;
use crate::rewards::Reward;
use crate::services::ErrorResponse;

#[derive(Serialize)]
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
