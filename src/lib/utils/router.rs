use axum::{
    routing::{get, post},
    Router,
};

use crate::services::{
    reward::redeem,
    status::status,
    user::{get_balance, register, reward},
};

/// Get the base path for the API.
pub fn get_base_path() -> String {
    format!("/api/{}", crate::VERSION)
}

/// Initialize the router for the API.
pub fn init_router() -> Router {
    let base_path = &get_base_path();
    Router::new()
        .route(&format!("{base_path}/status"), get(status))
        .route(&format!("{base_path}/user"), post(register))
        .route(&format!("{base_path}/user/:id/balance"), get(get_balance))
        .route(&format!("{base_path}/user/:id/reward"), post(reward))
        .route(&format!("{base_path}/reward/:id/redeem"), post(redeem))
}

/// Initialize the server for the API and listen on the specified address.
pub async fn init_server(bind_address: String) -> Result<(), std::io::Error> {
    // initialize our router and bind the address
    let app = init_router();
    let listener = tokio::net::TcpListener::bind(bind_address).await?;

    axum::serve(listener, app).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_base_path() {
        let base_path = get_base_path();

        assert!(base_path.contains("/api/v"));
    }
}
