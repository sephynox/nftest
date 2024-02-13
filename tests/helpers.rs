use std::io::Error;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

use axum::Router;
use dotenvy::dotenv;
use nftest::utils::router::get_base_path;
use nftest::utils::router::init_router;
use tokio::sync::OnceCell;
// use ethers::utils::Anvil;
use ethers::{abi::Abi, core::k256::ecdsa::SigningKey, prelude::*};
use lazy_static::lazy_static;
use nftest::core::chain::get_wallet_from_secret_key;

lazy_static! {
    static ref FUND_MUTEX: Mutex<()> = Mutex::new(());
    static ref DEPLOY_MUTEX: Mutex<()> = Mutex::new(());
}

static CONTRACT: OnceCell<Arc<Contract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>> =
    OnceCell::const_new();

static REWARD_ADDRESS: OnceCell<Address> = OnceCell::const_new();
static REWARD_NFT_ADDRESS: OnceCell<Address> = OnceCell::const_new();

/// We only want to start the server once, so we use a `OnceCell` to store the
/// socket address.
static TEST_SERVER: OnceCell<SocketAddr> = tokio::sync::OnceCell::const_new();

/// Get the reward token address.
pub fn get_reward_token_address() -> Option<Address> {
    REWARD_ADDRESS.get().map(|address| address.clone())
}

/// Get the reward NFT address.
pub fn get_reward_nft_address() -> Option<Address> {
    REWARD_NFT_ADDRESS.get().map(|address| address.clone())
}

/// Get the admin wallet.
pub fn get_admin_wallet() -> Result<Wallet<SigningKey>, Error> {
    dotenv().expect(".env file not found");

    let admin_key =
        std::env::var("PRIVATE_KEY").unwrap_or_else(|_| panic!("PRIVATE_KEY must be set"));
    let chain_id = std::env::var("CHAIN_ID").unwrap_or_else(|_| panic!("CHAIN_ID must be set"));
    let admin_wallet = get_wallet_from_secret_key(&admin_key)?;

    Ok(admin_wallet.with_chain_id(
        u64::from_str(&chain_id).unwrap_or_else(|_| panic!("CHAIN_ID must be a number")),
    ))
}

/// Fund the wallet with the specified amount from the admin wallet.
pub async fn fund_wallet(wallet: &Wallet<SigningKey>, amount: U256) -> Result<(), Error> {
    // Lock the mutex as we can only fund a wallet one at a time
    let _guard = FUND_MUTEX.lock().unwrap();

    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| panic!("RPC_URL must be set"));
    let provider = Provider::<Http>::try_from(rpc_url)
        .map_err(|_| Error::new(std::io::ErrorKind::Other, "Could not connect to RPC URL"))?;
    let admin_wallet = get_admin_wallet()?;

    // 1. Create a SignerMiddleware
    let client = SignerMiddleware::new(provider, admin_wallet);

    // 2. Create a TransactionRequest object
    let tx = TransactionRequest::new().to(wallet.address()).value(amount);

    // 3. Send the transaction with the client
    client
        .send_transaction(tx, None)
        .await
        .map_err(|_| Error::new(std::io::ErrorKind::Other, "Could not send transaction"))?
        .await
        .map_err(|_| Error::new(std::io::ErrorKind::Other, "Could not send transaction"))?;

    Ok(())
}

/// Start the server and return the socket address. It will bind to a random,
/// unused port which is why you need the SocketAddr returned.
async fn start_server(router: Router) -> SocketAddr {
    let bind_address = String::from("0.0.0.0:0");
    let listener = tokio::net::TcpListener::bind(bind_address)
        .await
        .expect("bind failed");

    let socket_addr = listener.local_addr().expect("could not get socket address");

    tokio::spawn(async move {
        axum::serve(listener, router.into_make_service())
            .await
            .unwrap();
    });

    socket_addr
}

/// Get the socket address for the test server. This will start the server if it
/// has not already been started.
async fn get_socket_addr(router: Router) -> SocketAddr {
    let addr = TEST_SERVER.get_or_init(|| start_server(router)).await;

    *addr
}

/// Get the base path for the test server. You should only need to use this
/// function for integration tests.
pub async fn get_test_base_path() -> String {
    let addr = get_socket_addr(init_router()).await;
    format!("http://{}{}", addr, get_base_path())
}

/// Get the provider for the test server.
pub fn get_provider() -> Provider<Http> {
    Provider::<Http>::try_from(
        std::env::var("RPC_URL").unwrap_or_else(|_| panic!("RPC_URL must be set")),
    )
    .unwrap()
}

pub async fn deploy_contracts() -> Result<(), Box<dyn std::error::Error>> {
    // Lock the mutex to prevent multiple deployments
    let _guard = DEPLOY_MUTEX.lock();

    dotenv().expect(".env file not found");

    // Check if the contract has already been deployed
    if CONTRACT.get().is_none() {
        // TODO
        // Spawn the anvil to start a local network
        // let anvil = Anvil::new().spawn();

        // let rpc_url = anvil.endpoint();
        // let chain_id = anvil.chain_id();

        // // Set the anvil environment variables
        // std::env::set_var("RPC_URL", rpc_url.clone());
        // std::env::set_var("CHAIN_ID", chain_id.to_string());

        // // Connect to the network
        // let provider = Provider::<Http>::try_from(rpc_url)?;

        // // Load the wallet
        // let wallet: LocalWallet = get_wallet_from_secret_key(
        //     &std::env::var("PRIVATE_KEY").unwrap_or_else(|_| panic!("PRIVATE_KEY must be set")),
        // )?
        // .with_chain_id(chain_id);

        // Get the Chain ID from the environment
        let chain_id = std::env::var("CHAIN_ID").unwrap_or_else(|_| panic!("CHAIN_ID must be set"));
        // Connect to the network
        let provider = get_provider();

        // Load the wallet
        let wallet: LocalWallet = get_wallet_from_secret_key(
            &std::env::var("PRIVATE_KEY").unwrap_or_else(|_| panic!("PRIVATE_KEY must be set")),
        )?
        .with_chain_id(
            u64::from_str(&chain_id).unwrap_or_else(|_| panic!("CHAIN_ID must be a number")),
        );

        // Connect the wallet to the provider
        let client = SignerMiddleware::new(provider, wallet);

        // Load the contract ABI
        let contract_json = include_str!("../out/Deploy.s.sol/DeployScript.json");
        let contract_data: serde_json::Value = serde_json::from_str(contract_json).unwrap();
        let abi: Abi = serde_json::from_value(contract_data["abi"].clone()).unwrap();

        // Load the contract bytecode from the ABI
        let bytecode_str = contract_data["bytecode"]["object"].as_str().unwrap();
        let bytecode = Bytes::from_str(bytecode_str)?;

        // Create a factory for the contract
        let factory = ContractFactory::new(abi, bytecode, client.into());
        // Deploy the contract
        let contract = factory.deploy(())?.send().await?;

        // Store the contract in the OnceCell
        CONTRACT.set(Arc::new(contract)).unwrap();

        // Call the run function to run the script
        CONTRACT
            .get()
            .unwrap()
            .method::<_, ()>("deploy", ())?
            .send()
            .await?;

        // Call the getter functions and store the addresses
        let reward_address: Address = CONTRACT
            .get()
            .unwrap()
            .method::<_, Address>("getRewardAddress", ())?
            .call()
            .await?;
        let reward_nft_address: Address = CONTRACT
            .get()
            .unwrap()
            .method::<_, Address>("getRewardNFTAddress", ())?
            .call()
            .await?;

        // Set the addresses as environment variables
        std::env::set_var("REWARD_TOKEN_ADDRESS", format!("{:#x}", reward_address));
        std::env::set_var("REWARD_NFT_ADDRESS", format!("{:#x}", reward_nft_address));

        REWARD_ADDRESS.set(reward_address).unwrap();
        REWARD_NFT_ADDRESS.set(reward_nft_address).unwrap();
    }

    // Unlock the mutex
    drop(_guard);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::{
        core::rand::thread_rng,
        providers::{Http, Provider},
    };
    use std::convert::TryFrom;

    #[tokio::test]
    async fn test_deploy_contracts() {
        dotenv().expect(".env file not found");

        // Get the RPC URL from the environment
        let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| panic!("RPC_URL must be set"));
        // Call the function
        let result = deploy_contracts().await;

        // Check if the function returned Ok
        assert!(result.is_ok());

        // Check if the contract was deployed
        let contract = CONTRACT.get().unwrap();
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();
        let code = provider.get_code(contract.address(), None).await.unwrap();

        // Check if the contract was deployed
        assert!(!code.0.is_empty());
        // Check if the addresses were stored
        assert!(REWARD_ADDRESS.get().is_some());
        assert!(REWARD_NFT_ADDRESS.get().is_some());
        // Check the environment variables
        assert_eq!(
            std::env::var("REWARD_TOKEN_ADDRESS").unwrap(),
            format!("{:#x}", REWARD_ADDRESS.get().unwrap())
        );
        assert_eq!(
            std::env::var("REWARD_NFT_ADDRESS").unwrap(),
            format!("{:#x}", REWARD_NFT_ADDRESS.get().unwrap())
        );
    }

    #[tokio::test]
    async fn test_fund_wallet() {
        // Deploy the contracts
        deploy_contracts().await.unwrap();

        // Get the provider
        let provider = get_provider();

        // Create a new empty local wallet
        let wallet = LocalWallet::new(&mut thread_rng())
            .with_chain_id(u64::from_str(&std::env::var("CHAIN_ID").unwrap()).unwrap());

        let address = wallet.address();

        // Fund the wallet
        let value: u128 = 1000000000000000000;
        let amount = U256::from(value);
        let result = fund_wallet(&wallet, amount).await;

        // Check the result
        assert!(result.is_ok());

        let balance = provider.get_balance(address, None).await.unwrap();

        // Check that the wallet balance is correct
        assert_eq!(balance, U256::from(value));
    }
}
