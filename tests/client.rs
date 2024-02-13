use std::str::FromStr;

use ethers::core::rand::thread_rng;
use ethers::signers::LocalWallet;
use ethers::signers::Signer;
use ethers::types::U256;
use nftest::core::chain::burn_nft_reward;
use nftest::core::chain::get_reward_balance;
use nftest::core::chain::get_reward_nft_contract;
use nftest::core::chain::get_reward_token_contract;
use nftest::core::chain::get_wallet_from_secret_key;
use nftest::core::chain::mint_nft_reward;
use nftest::utils::helpers::random_u256;

mod helpers;

#[tokio::test]
async fn test_get_reward_token_contract() {
    // Deploy the contracts
    helpers::deploy_contracts().await.unwrap();

    // Get the reward token contract
    let contract = get_reward_token_contract().unwrap();

    // Check that the contract address is correct
    assert_eq!(
        contract.address(),
        helpers::get_reward_token_address().unwrap()
    );
}

#[tokio::test]
async fn test_get_reward_nft_contract() {
    // Deploy the contracts
    helpers::deploy_contracts().await.unwrap();

    // Get the reward token contract
    let contract = get_reward_nft_contract().unwrap();

    // Check that the contract address is correct
    assert_eq!(
        contract.address(),
        helpers::get_reward_nft_address().unwrap()
    );
}

#[tokio::test]
async fn test_get_reward_balance() {
    // Deploy the contracts
    helpers::deploy_contracts().await.unwrap();

    // Create a new wallet
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let wallet = get_wallet_from_secret_key(private_key).unwrap();
    // Test the get_reward_balance function
    let result = get_reward_balance(&wallet).await;

    assert!(result.is_ok());

    // Check that the balance is correct
    assert_eq!(result.unwrap(), U256::from(0));
}

#[tokio::test]
async fn test_mint_nft_reward() {
    // Deploy the contracts
    helpers::deploy_contracts().await.unwrap();

    // Define the parameters for the mint_nft_reward function
    let wallet = LocalWallet::new(&mut thread_rng());
    let to = wallet.address();
    let token_id = random_u256();
    let url = "https://example.com".to_string();
    let value = U256::from(1000);

    // Call the mint_nft_reward function
    let result = mint_nft_reward(to, token_id, url, value).await;

    // Assert that the function returned Ok
    assert!(result.is_ok());

    // TODO Check that the NFT was minted to the correct address
}

#[tokio::test]
async fn test_burn_nft_reward() {
    // Deploy the contracts
    helpers::deploy_contracts().await.unwrap();

    // Define the parameters for the mint_nft_reward function
    let wallet = LocalWallet::new(&mut thread_rng())
        .with_chain_id(u64::from_str(&std::env::var("CHAIN_ID").unwrap()).unwrap());
    let value: u128 = 1000000000000000000;
    let amount = U256::from(value);

    // Fund the wallet so it can pay for the transaction gas
    helpers::fund_wallet(&wallet, amount).await.unwrap();

    let to = wallet.address();
    let token_id = random_u256();
    let url = "https://example.com".to_string();
    let value = U256::from(1000);

    // Call the mint_nft_reward function
    let result = mint_nft_reward(to, token_id, url, value).await;

    // Assert that the function returned Ok
    assert!(result.is_ok());

    // Call the burn_nft_reward function
    let result = burn_nft_reward(wallet, token_id).await;

    // Assert that the function returned Ok
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_full_flow() {
    // Deploy the contracts
    helpers::deploy_contracts().await.unwrap();
    // Start a new test server
    let api_path = helpers::get_test_base_path().await;
}
