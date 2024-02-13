use std::io::Error;
use std::str::FromStr;
use std::sync::Arc;

use dotenvy::dotenv;
use ethers::abi::{Abi, Address};
use ethers::contract::{Contract, ContractInstance, FunctionCall};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::k256::SecretKey;
use ethers::core::rand;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer, Wallet};
use ethers::types::U256;
use hex::FromHexError;

/// Simple function to generate a new secret key
pub fn generate_secret_key() -> String {
    let wallet = Wallet::new(&mut rand::thread_rng());
    let private_key = wallet
        .signer()
        .to_bytes()
        .iter()
        // Pad the hexadecimal representation with a leading zero if the byte
        // value is less than 16 (0x10)
        .map(|&i| format!("{:02X}", i))
        .collect::<Vec<String>>()
        .join("");

    format!("0x{}", private_key)
}

/// Converts a hex string to a byte array
pub fn get_key_bytes(key: &str) -> Result<Vec<u8>, FromHexError> {
    let key = key.strip_prefix("0x").unwrap_or(key);
    hex::decode(key)
}

/// Converts a secret key to a wallet
/// TODO Improve error handling
pub fn get_wallet_from_secret_key(secret_key: &str) -> Result<LocalWallet, Error> {
    let secret_key =
        get_key_bytes(secret_key).map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
    let secret_key = SecretKey::from_slice(&secret_key)
        .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
    let local_wallet = LocalWallet::from(secret_key);

    Ok(local_wallet)
}

fn get_admin_wallet() -> Result<Wallet<SigningKey>, Error> {
    dotenv().expect(".env file not found");

    let admin_key =
        std::env::var("PRIVATE_KEY").unwrap_or_else(|_| panic!("PRIVATE_KEY must be set"));
    let chain_id = std::env::var("CHAIN_ID").unwrap_or_else(|_| panic!("CHAIN_ID must be set"));
    let admin_wallet = get_wallet_from_secret_key(&admin_key)?;

    Ok(admin_wallet.with_chain_id(
        u64::from_str(&chain_id).unwrap_or_else(|_| panic!("CHAIN_ID must be a number")),
    ))
}

/// Mint a new NFT reward
pub async fn mint_nft_reward(
    to: Address,
    token_id: U256,
    url: String,
    value: U256,
) -> Result<String, Error> {
    // Get the admin wallet
    let wallet = get_admin_wallet()?;
    // Get the reward NFT contract
    let contract = get_reward_nft_contract()?;
    // Get the provider
    let provider = get_provider()?;
    // Create a new signer middleware
    let client = SignerMiddleware::new(provider, wallet);

    // Mint the reward NFT
    let call: FunctionCall<
        Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        (
            Address,
            ethers::types::U256,
            std::string::String,
            ethers::types::U256,
        ),
    > = contract
        // Connect the contract to the provider to use the signer middleware
        .connect(client.into())
        // Specify the safeMint function of the contract
        .method("safeMint", (to, token_id, url, value))
        .map_err(|e| {
            Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to prepare minting call: {:?}", e),
            )
        })?;

    let tx = call.send().await.map_err(|e| {
        Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to mint reward: {:?}", e),
        )
    })?;

    Ok(tx.tx_hash().to_string())
}

/// Redeem an NFT reward
pub async fn redeem_nft_reward(token_id: U256) -> Result<String, Error> {
    todo!()
}

fn get_provider() -> Result<Provider<Http>, Error> {
    dotenv().expect(".env file not found");

    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| panic!("RPC_URL must be set"));
    Provider::<Http>::try_from(rpc_url).map_err(|e| {
        Error::new(
            std::io::ErrorKind::ConnectionRefused,
            format!("Failed to connect to provider: {:?}", e),
        )
    })
}

/// Creates a new contract instance from the given contract JSON and address
fn create_contract_instance(
    contract_json: &str,
    contract_address: Address,
) -> Result<ContractInstance<Arc<Provider<Http>>, Provider<Http>>, Error> {
    // Get the provider
    let provider = get_provider()?;
    // Load the contract ABI
    let contract_data: serde_json::Value = serde_json::from_str(contract_json).unwrap();
    let abi: Abi = serde_json::from_value(contract_data["abi"].clone()).unwrap();

    Ok(Contract::new(contract_address, abi, provider.into()))
}

/// Get the contract address from the environment
fn get_contract_address_from_env(env_var: &str) -> Address {
    let contract_address_str =
        std::env::var(env_var).unwrap_or_else(|_| panic!("{} must be set", env_var));
    Address::from_str(&contract_address_str)
        .unwrap_or_else(|_| panic!("{} must be a valid address", env_var))
}

/// Get the reward token contract
pub fn get_reward_token_contract(
) -> Result<ContractInstance<Arc<Provider<Http>>, Provider<Http>>, Error> {
    let contract_json = include_str!("../../../out/Reward.sol/Reward.json");
    let contract_address = get_contract_address_from_env("REWARD_TOKEN_ADDRESS");
    create_contract_instance(contract_json, contract_address)
}

/// Get the reward NFT contract
pub fn get_reward_nft_contract(
) -> Result<ContractInstance<Arc<Provider<Http>>, Provider<Http>>, Error> {
    let contract_json = include_str!("../../../out/RewardNFT.sol/RewardNFT.json");
    let contract_address = get_contract_address_from_env("REWARD_NFT_ADDRESS");
    create_contract_instance(contract_json, contract_address)
}

/// Get the reward balance of a wallet
/// TODO Improve error handling
pub async fn get_reward_balance(wallet: &Wallet<SigningKey>) -> Result<U256, Error> {
    // Get the wallet address
    let wallet_address = wallet.address();
    // Create a new contract instance
    let contract = get_reward_token_contract()?;
    let contract_method = contract.method("balanceOf", wallet_address).map_err(|e| {
        Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to get balance: {:?}", e),
        )
    })?;

    // Call the balanceOf function
    let balance: U256 = contract_method.call().await.map_err(|e| {
        Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to get balance: {:?}", e),
        )
    })?;

    Ok(balance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret_key() {
        let key = generate_secret_key();

        // Check that the key is not empty
        assert!(!key.is_empty(), "Generated key should not be empty");
        // Check that the key starts with "0x"
        assert!(key.starts_with("0x"), "Key does not start with '0x'");

        let rest = &key[2..];

        // Check that the rest of the key is 64 characters long
        assert_eq!(rest.len(), 64, "Key is not the correct length");
        // Check that the rest of the key is hexadecimal
        assert!(
            rest.chars().all(|c| c.is_digit(16)),
            "Key contains non-hexadecimal characters"
        );
    }

    #[test]
    fn test_get_key_bytes() {
        let key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let bytes = get_key_bytes(key).unwrap();

        assert_eq!(bytes.len(), 32, "Byte array is not the correct length");

        let key_without_prefix = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let bytes_without_prefix = get_key_bytes(key_without_prefix).unwrap();

        assert_eq!(
            bytes_without_prefix.len(),
            32,
            "Byte array is not the correct length"
        );
        assert_eq!(bytes, bytes_without_prefix, "Byte arrays are not equal");
    }

    #[tokio::test]
    async fn test_get_wallet_from_secret_key() {
        let secret_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let wallet = get_wallet_from_secret_key(secret_key).unwrap();
        let expected_address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        let h160 = wallet.address();

        // Check that the wallet's address is correct
        assert_eq!(format!("{h160:#020x}"), expected_address);

        let message = "Hello, world!";
        let signature = wallet.sign_message(message).await.unwrap();

        // Check that the wallet can verify the signature
        assert_eq!(signature.recover(&message[..]).unwrap(), wallet.address());
    }
}
