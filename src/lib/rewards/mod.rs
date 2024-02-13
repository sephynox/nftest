use ethers::{prelude::LocalWallet, types::U256};
use uuid::Uuid;

/// Reward is a trait that must be implemented by all rewards.
#[async_trait::async_trait]
pub trait Reward {
    type Error;

    fn get_value(&self) -> U256;
    fn get_url(&self) -> String;
    fn get_id(&self) -> String;
    fn get_owner(&self) -> Uuid;
    fn is_redeemed(&self) -> bool;
    async fn mint(value: U256, wallet: &LocalWallet) -> Result<U256, Self::Error>;
    async fn redeem(&mut self) -> Result<U256, Self::Error>;
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::utils::helpers::random_u256;

    use super::*;

    struct TestReward {
        id: String,
        owner: Uuid,
        value: U256,
        url: String,
        redeemed: bool,
    }

    #[async_trait::async_trait]
    impl Reward for TestReward {
        type Error = std::io::Error;

        fn get_id(&self) -> String {
            self.id.clone()
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

        async fn mint(_value: U256, _wallet: &LocalWallet) -> Result<U256, Self::Error> {
            Ok(random_u256())
        }

        async fn redeem(&mut self) -> Result<U256, Self::Error> {
            self.redeemed = true;
            Ok(random_u256())
        }
    }

    #[test]
    fn test_reward() {
        let owner_id = Uuid::new_v4();
        let reward = TestReward {
            id: "test".to_string(),
            owner: owner_id.clone(),
            value: U256::from(100),
            url: "http://example.com".to_string(),
            redeemed: false,
        };

        assert_eq!(reward.get_id(), "test".to_string());
        assert_eq!(reward.get_value(), U256::from(100));
        assert_eq!(reward.get_url(), "http://example.com".to_string());
        assert_eq!(reward.get_owner(), owner_id);
        assert_eq!(reward.is_redeemed(), false);
    }
}
