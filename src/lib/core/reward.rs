use thiserror::Error;

use super::repository::RepositoryError;

pub trait Reward {
    fn get_value(&self) -> u128;
    fn redeem(&self, _reward: u128) -> bool;
}

pub trait Owned {
    fn owner(&self) -> &str;
}

#[derive(Debug, Error)]
pub enum RewardError {
    #[error("Reward not found")]
    NotFound,
    #[error("Reward already exists")]
    AlreadyExists,
    #[error("Reward already redeemed")]
    AlreadyRedeemed,
    #[error("Repository error")]
    RepositoryError(#[from] RepositoryError),
    #[error("Failed to mint reward")]
    MintRewardError,
    #[error("Unknown error")]
    UnknownError(#[from] std::io::Error),
}
