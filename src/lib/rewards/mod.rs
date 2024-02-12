use ethers::prelude::LocalWallet;

/// Reward is a trait that must be implemented by all rewards.
#[async_trait::async_trait]
pub trait Reward {
    type Error;

    fn get_value(&self) -> u128;
    fn get_url(&self) -> String;
    fn get_id(&self) -> String;
    async fn mint(&self, wallet: &LocalWallet) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestReward {
        id: String,
        value: u128,
        url: String,
    }

    #[async_trait::async_trait]
    impl Reward for TestReward {
        type Error = std::io::Error;

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
            Ok(())
        }
    }

    #[test]
    fn test_reward() {
        let reward = TestReward {
            id: "test".to_string(),
            value: 100,
            url: "http://example.com".to_string(),
        };

        assert_eq!(reward.get_id(), "test".to_string());
        assert_eq!(reward.get_value(), 100);
        assert_eq!(reward.get_url(), "http://example.com".to_string());
    }
}
