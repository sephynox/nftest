use ethers::{
    core::{rand, rand::Rng},
    types::U256,
};

/// Generate a random U256
pub fn random_u256() -> U256 {
    let mut rng = rand::thread_rng();
    let lower = rng.gen::<u128>();
    let upper = rng.gen::<u128>();
    let upper = U256::from(upper) << 128;
    let lower = U256::from(lower);

    upper + lower
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_u256() {
        let a = random_u256();
        let b = random_u256();
        let c = random_u256();

        assert_ne!(a, b, "Two random U256 numbers are equal");
        assert_ne!(a, c, "Two random U256 numbers are equal");
        assert_ne!(b, c, "Two random U256 numbers are equal");
    }
}
