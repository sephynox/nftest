pub mod example;

/// Reward is a trait that must be implemented by all rewards.
pub trait Reward {
    fn get_value(&self) -> u128;
    fn get_url(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestReward {
        value: u128,
        url: String,
    }

    impl Reward for TestReward {
        fn get_value(&self) -> u128 {
            self.value
        }

        fn get_url(&self) -> String {
            self.url.clone()
        }
    }

    #[test]
    fn test_reward() {
        let reward = TestReward {
            value: 100,
            url: "http://example.com".to_string(),
        };

        assert_eq!(reward.get_value(), 100);
        assert_eq!(reward.get_url(), "http://example.com".to_string());
    }
}
