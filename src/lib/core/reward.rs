pub trait Reward {
    fn get_value(&self) -> u128;
    fn redeem(&self, _reward: u128) -> bool;
}

pub trait Owned {
    fn owner(&self) -> &str;
}