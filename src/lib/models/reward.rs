use ethers::{signers::LocalWallet, types::U256};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    core::{
        repository::{Repository, RepositoryError},
        reward::RewardError,
    },
    rewards::Reward,
    storage::sled::{get_sled_db, SledModel},
    utils::helpers::random_u256,
};

use super::user::User;

/// TODO Improve
const REWARD_NFT_URL: &str = "https://localhost:3001/api/v1/reward";

/// A simple reward that can be redeemed.
#[derive(Clone, Serialize, Deserialize)]
pub struct RewardNFT {
    /// The id of the reward.
    id: Uuid,
    /// The owner of the reward.
    owner: Uuid,
    /// The token id of the reward.
    token_id: U256,
    /// The value of the reward.
    value: U256,
    /// The url of NFT reward data.
    url: String,
    /// If the reward is redeemed.
    redeemed: bool,
}

impl RewardNFT {
    /// Create a new reward.
    pub fn new(owner: User, value: U256, token_id: U256) -> Self {
        let id = Uuid::new_v4();
        let url = format!("{}/{}", REWARD_NFT_URL, id);
        let redeemed = false;
        let owner = owner.id;

        Self {
            id,
            owner,
            token_id,
            value,
            url,
            redeemed,
        }
    }

    pub async fn from_id(id: String) -> Result<Self, RewardError> {
        let connection = get_sled_db()?;
        let db = connection
            .read()
            .map_err(|_| RewardError::RepositoryError(RepositoryError::ConnectionError))?;

        let reward: Result<Option<RewardNFT>, RepositoryError> = db.read(id);

        match reward {
            Ok(Some(reward)) => Ok(reward),
            Ok(None) => Err(RewardError::NotFound),
            Err(e) => Err(RewardError::RepositoryError(e)),
        }
    }

    /// Save the reward to the repository.
    pub async fn save(&self, create: bool) -> Result<(), RewardError> {
        let connection = get_sled_db()?;
        let db = connection
            .write()
            .map_err(|_| RewardError::RepositoryError(RepositoryError::ConnectionError))?;

        // Check if the reward already exists
        let existing_reward: Result<Option<RewardNFT>, RepositoryError> =
            db.read(self.id.to_string());

        match existing_reward {
            Ok(Some(_)) if create => Err(RewardError::AlreadyExists),
            Ok(Some(_)) if !create => {
                db.update(self.id.to_string(), self.clone())
                    .map_err(|_| RewardError::RepositoryError(RepositoryError::UpdateError))?;
                Ok(())
            }
            Ok(None) if create => {
                db.create(self.id.to_string(), self.clone())
                    .map_err(|_| RewardError::RepositoryError(RepositoryError::InsertionError))?;
                Ok(())
            }
            _ => Err(RewardError::RepositoryError(
                RepositoryError::InsertionError,
            )),
        }
    }
}

#[async_trait::async_trait]
impl Reward for RewardNFT {
    type Error = RewardError;

    fn get_id(&self) -> String {
        self.id.to_string()
    }

    fn get_value(&self) -> U256 {
        self.value
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }

    fn get_owner(&self) -> Uuid {
        self.owner.clone()
    }

    fn is_redeemed(&self) -> bool {
        self.redeemed
    }

    #[cfg(not(test))]
    async fn mint(value: U256, wallet: &LocalWallet) -> Result<U256, Self::Error> {
        use ethers::signers::Signer;

        let token_id = random_u256();
        let url = format!("{}/{}", REWARD_NFT_URL, token_id);

        crate::core::chain::mint_nft_reward(wallet.address(), token_id, url, value)
            .await
            .map_err(|_| RewardError::MintRewardError)?;

        Ok(token_id)
    }

    #[cfg(test)]
    async fn mint(_value: U256, _wallet: &LocalWallet) -> Result<U256, Self::Error> {
        Ok(random_u256())
    }

    #[cfg(not(test))]
    async fn redeem(&mut self) -> Result<U256, Self::Error> {
        if self.redeemed {
            Err(RewardError::AlreadyRedeemed)
        } else {
            // Get the user from the repository
            let user = User::from_id(self.owner.to_string())
                .await
                .map_err(|_| RewardError::RepositoryError(RepositoryError::ConnectionError))?;
            // Get the user's wallet
            let wallet = user.get_wallet()?;

            crate::core::chain::burn_nft_reward(wallet, self.token_id)
                .await
                .map_err(|_| RewardError::MintRewardError)?;

            self.redeemed = true;

            // Save the reward state to the repository
            self.save(false)
                .await
                .map_err(|_| RewardError::RepositoryError(RepositoryError::UpdateError))?;

            Ok(self.value)
        }
    }

    #[cfg(test)]
    async fn redeem(&mut self) -> Result<U256, Self::Error> {
        self.redeemed = true;

        // Save the reward state to the repository
        self.save(false)
            .await
            .map_err(|_| RewardError::RepositoryError(RepositoryError::UpdateError))?;

        Ok(self.value)
    }
}

impl SledModel for RewardNFT {}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_reward(value: u128) -> RewardNFT {
        let owner = User::new(Uuid::new_v4(), "test".to_string());
        RewardNFT::new(owner, U256::from(value), random_u256())
    }

    #[test]
    fn test_new() {
        let reward = generate_reward(100);

        assert_eq!(reward.id.to_string().len(), 36);
        assert_eq!(reward.value, U256::from(100));
        assert_eq!(reward.url, format!("{}/{}", REWARD_NFT_URL, reward.id));
    }

    #[test]
    fn test_get_value() {
        let reward = generate_reward(100);

        assert_eq!(reward.get_value(), U256::from(100));
    }

    #[test]
    fn test_get_url() {
        let reward = generate_reward(100);

        assert_eq!(
            reward.get_url(),
            format!("{}/{}", REWARD_NFT_URL, reward.id)
        );
    }

    #[tokio::test]
    async fn test_save() {
        let reward = generate_reward(100);

        // Save the reward for the first time
        assert!(reward.save(true).await.is_ok());

        // Try to save the same reward again
        match reward.save(true).await {
            Ok(_) => panic!("Expected error, but got Ok"),
            Err(e) => match e {
                RewardError::AlreadyExists => {}
                _ => panic!("Expected AlreadyExists, but got {:?}", e),
            },
        }
    }

    #[tokio::test]
    async fn test_from_id() {
        let id = Uuid::new_v4().to_string();
        let result = RewardNFT::from_id(id).await;

        // A non existing reward should return an error
        assert!(result.is_err());

        // Create a new reward
        let reward = generate_reward(100);

        // Save the reward for the first time
        assert!(reward.save(true).await.is_ok());

        let result = RewardNFT::from_id(reward.id.to_string()).await;

        // An existing reward should return Ok
        assert!(result.is_ok());

        let result = result.unwrap();

        // Check that the reward is the same
        assert_eq!(result.id, reward.id);
        assert_eq!(result.value, reward.value);
        assert_eq!(result.url, reward.url);
    }

    #[tokio::test]
    async fn test_redeem() {
        let mut reward = generate_reward(100);

        // Save the reward
        assert!(reward.save(true).await.is_ok());

        let value = reward.redeem().await.unwrap();

        // Check that the reward was redeemed
        assert_eq!(value, reward.value);
        // Check that the reward is redeemed
        assert!(reward.redeemed);
        assert!(reward.is_redeemed());
    }
}
