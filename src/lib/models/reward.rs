use ethers::signers::LocalWallet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    core::{
        repository::{Repository, RepositoryError},
        reward::RewardError,
    },
    rewards::Reward,
    storage::sled::{get_sled_db, SledModel},
};

/// TODO Improve
const REWARD_NFT_URL: &str = "https://localhost:3001/api/v1/reward";

/// A simple reward that can be redeemed.
#[derive(Clone, Serialize, Deserialize)]
pub struct RewardNFT {
    /// The id of the reward.
    id: String,
    /// The value of the reward.
    value: u128,
    /// The url of NFT reward data.
    url: String,
}

impl RewardNFT {
    /// Create a new reward.
    pub fn new(value: u128) -> Self {
        let id = Uuid::new_v4().to_string();
        let url = format!("{}/{}", REWARD_NFT_URL, id);

        Self { id, value, url }
    }

    /// Save the reward to the repository.
    pub async fn save(&self) -> Result<(), RewardError> {
        let connection = get_sled_db()?;
        let db = connection
            .read()
            .map_err(|_| RewardError::RepositoryError(RepositoryError::ConnectionError))?;

        // Check if the reward already exists
        let existing_reward: Result<Option<RewardNFT>, RepositoryError> = db.read(self.id.clone());

        if existing_reward.is_ok() && existing_reward.unwrap().is_some() {
            Err(RewardError::AlreadyExists)
        } else {
            db.create(self.id.clone(), self.clone())
                .map_err(|_| RewardError::RepositoryError(RepositoryError::InsertionError))?;

            Ok(())
        }
    }
}

#[async_trait::async_trait]
impl Reward for RewardNFT {
    type Error = RewardError;

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_value(&self) -> u128 {
        self.value
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }

    async fn mint(&self, _wallet: &LocalWallet) -> Result<(), Self::Error> {
        self.save().await?;
        Ok(())
    }
}

impl SledModel for RewardNFT {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let reward = RewardNFT::new(100);

        assert_eq!(reward.id.len(), 36);
        assert_eq!(reward.value, 100);
        assert_eq!(reward.url, format!("{}/{}", REWARD_NFT_URL, reward.id));
    }

    #[test]
    fn test_get_value() {
        let reward = RewardNFT::new(100);

        assert_eq!(reward.get_value(), 100);
    }

    #[test]
    fn test_get_url() {
        let reward = RewardNFT::new(100);

        assert_eq!(
            reward.get_url(),
            format!("{}/{}", REWARD_NFT_URL, reward.id)
        );
    }
}
