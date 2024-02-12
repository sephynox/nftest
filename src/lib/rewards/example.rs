use super::Reward;

/// A simple reward that can be redeemed.
pub struct RewardNFT {
    /// The value of the reward.
    value: u128,
    /// The url of NFT reward data.
    url: String,
}

impl RewardNFT {
    /// Create a new reward.
    pub fn new(value: u128, url: String) -> Self {
        Self { value, url }
    }
}

impl Reward for RewardNFT {
    fn get_value(&self) -> u128 {
        self.value
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let reward = RewardNFT::new(100, "https://example.com".to_string());

        assert_eq!(reward.value, 100);
        assert_eq!(reward.url, "https://example.com");
    }

    #[test]
    fn test_get_value() {
        let reward = RewardNFT::new(100, "https://example.com".to_string());

        assert_eq!(reward.get_value(), 100);
    }

    #[test]
    fn test_get_url() {
        let reward = RewardNFT::new(100, "https://example.com".to_string());

        assert_eq!(reward.get_url(), "https://example.com");
    }
}
