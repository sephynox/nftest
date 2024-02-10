use std::error::Error;
use std::env;

use dotenvy::dotenv;

use nftest::utils::router::init_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().expect(".env file not found");

    // get the port from the environment, or use 3001 if it's not set
    let api_port = env::var("API_PORT").unwrap_or_else(|_| "3001".into());
    // create a string for our bind address
    let bind_address = format!("0.0.0.0:{api_port}");

	Ok(init_server(bind_address).await?)
}