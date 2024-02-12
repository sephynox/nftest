use std::io::Error;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::k256::SecretKey;
use ethers::core::rand;
use ethers::signers::{LocalWallet, Wallet};
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
pub fn get_wallet_from_secret_key(secret_key: &str) -> Result<Wallet<SigningKey>, Error> {
    let secret_key =
        get_key_bytes(secret_key).map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
    let secret_key = SecretKey::from_slice(&secret_key)
        .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
    let local_wallet = LocalWallet::from(secret_key);

    Ok(local_wallet.into())
}

#[cfg(test)]
mod tests {
    use ethers::signers::Signer;

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
