use std::io::Error;
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
use nftest::services::reward::RedeemResult;
use nftest::services::user::BalanceResult;
use nftest::services::user::RegisterRequest;
use nftest::services::user::RewardRequest;
use nftest::services::user::RewardResult;
use nftest::utils::helpers::random_u256;
use uuid::Uuid;

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
    let result = get_reward_balance(wallet.address()).await;

    assert!(result.is_ok());

    // Check that the balance is correct
    assert_eq!(result.unwrap(), U256::from(0));
}

async fn setup_mint_tests() -> LocalWallet {
    // Deploy the contracts
    helpers::deploy_contracts().await.unwrap();

    // Create a new wallet
    let wallet = LocalWallet::new(&mut thread_rng())
        .with_chain_id(u64::from_str(&std::env::var("CHAIN_ID").unwrap()).unwrap());

    // Fund the wallet so it can pay for the transaction gas
    let value: u128 = 1000000000000000000;
    let amount = U256::from(value);
    helpers::fund_wallet(&wallet, amount).await.unwrap();

    wallet
}

async fn test_mint_nft(wallet: &LocalWallet) -> Result<U256, Error> {
    let to = wallet.address();
    let token_id = random_u256();
    let url = "https://example.com".to_string();
    let value = U256::from(1000);

    // Call the mint_nft_reward function
    mint_nft_reward(to, token_id, url, value).await.unwrap();

    Ok(token_id)
}

#[tokio::test]
async fn test_mint_nft_reward() {
    // Setup the tests
    let wallet = setup_mint_tests().await;
    // Call the mint_nft_reward function
    let result = test_mint_nft(&wallet).await;

    // Assert that the function returned Ok
    assert!(result.is_ok());

    // TODO Check that the NFT was minted to the correct address
}

#[tokio::test]
async fn test_burn_nft_reward() {
    // Setup the tests
    let wallet = setup_mint_tests().await;
    // Get the address of the wallet
    let address = wallet.address();
    // Call the mint_nft_reward function
    let result = test_mint_nft(&wallet).await;

    // Assert that the function returned Ok
    assert!(result.is_ok());

    // Get the token id
    let token_id = result.unwrap();
    // Call the burn_nft_reward function
    let result = burn_nft_reward(wallet, token_id).await;

    // Assert that the function returned Ok
    assert!(result.is_ok());

    let balance = get_reward_balance(address).await;

    // Check that the balance is correct
    assert_eq!(balance.unwrap(), U256::from(1000));
}

#[tokio::test]
async fn test_full_flow() {
    // Deploy the contracts
    helpers::deploy_contracts().await.unwrap();
    // Start a new test server
    let api_path = helpers::get_test_base_path().await;
    let client = reqwest::Client::new();

    // Register a new user
    let user_id = Uuid::new_v4();
    let request = RegisterRequest { id: user_id };
    let result = client
        .post(&format!("{}/user", api_path))
        .json(&request)
        .send()
        .await;

    // Ensure the registration was successful
    assert!(result.is_ok());

    // Check the balance of the user
    let result = client
        .get(&format!("{}/user/{}/balance", api_path, user_id))
        .send()
        .await;

    // Ensure the request was successful
    assert!(result.is_ok());

    let result = result.unwrap().json::<BalanceResult>().await.unwrap();

    // Check that the balance is zero for a new user
    assert_eq!(result.balance, String::from("0"));

    // Reward the user with an NFT
    let value = 1337;
    let request = RewardRequest { value };
    let result = client
        .post(&format!("{}/user/{}/reward", api_path, user_id))
        .json(&request)
        .send()
        .await;

    // Ensure the request was successful
    assert!(result.is_ok());

    let result = result.unwrap().json::<RewardResult>().await.unwrap();
    let reward_id = Uuid::from_str(&result.id).unwrap();

    // Check that the reward was successful
    assert!(result.success);

    // Redeem the reward
    let result = client
        .post(&format!("{}/reward/{}/redeem", api_path, reward_id))
        .send()
        .await;

    // Ensure the request was successful
    assert!(result.is_ok());

    let result = result.unwrap().json::<RedeemResult>().await.unwrap();

    // Check that the correct reward was redeemed
    assert_eq!(result.id, reward_id);
    // Check that the correct value was returned
    assert_eq!(result.reward, value.to_string());

    // Check the new balance of the user
    let result = client
        .get(&format!("{}/user/{}/balance", api_path, user_id))
        .send()
        .await;

    // Ensure the request was successful
    assert!(result.is_ok());

    let result = result.unwrap().json::<BalanceResult>().await.unwrap();

    // Check that the balance has been updated with the redeemed reward
    assert_eq!(result.balance, value.to_string());
}
