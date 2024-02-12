use serde::{Deserialize, Serialize};
use thiserror::Error;
use zeroize::ZeroizeOnDrop;

use crate::core::repository::{Repository, RepositoryError};
use crate::storage::sled::{get_sled_db, SledModel};

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
    #[error("Repository error")]
    RepositoryError(#[from] RepositoryError),
}

/// User is a struct that contains the user's id and address.
#[derive(Clone, Serialize, Deserialize, ZeroizeOnDrop)]
pub struct User {
    /// User id
    #[zeroize(skip)]
    pub id: String,
    /// Private key
    pub key: String,
}

impl User {
    /// Create a new user.
    pub fn new(id: String, key: String) -> Self {
        Self { id, key }
    }

    /// Save the user to the repository.
    pub async fn save(&self) -> Result<(), UserError> {
        let connection = get_sled_db()?;
        let db = connection
            .read()
            .map_err(|_| UserError::RepositoryError(RepositoryError::ConnectionError))?;

        // Check if the user already exists
        let existing_user: Result<Option<User>, RepositoryError> = db.read(self.id.clone());

        if existing_user.is_ok() && existing_user.unwrap().is_some() {
            Err(UserError::AlreadyExists)
        } else {
            db.create(self.id.clone(), self.clone())
                .map_err(|_| UserError::RepositoryError(RepositoryError::InsertionError))?;

            Ok(())
        }
    }

    /// Look up a user by id from the repository.
    /// Note: We keep this async because it can be changed in the future
    /// to use a different repository.
    pub async fn from_id(id: String) -> Result<Self, UserError> {
        let connection = get_sled_db()?;
        let db = connection
            .read()
            .map_err(|_| UserError::RepositoryError(RepositoryError::ConnectionError))?;
        let user: Result<Option<User>, RepositoryError> = db.read(id);

        match user {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(UserError::NotFound),
            Err(e) => Err(UserError::RepositoryError(e)),
        }
    }
}

impl SledModel for User {}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use super::*;

    const PRIVATE_KEY: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    fn setup() -> (String, User) {
        let user_key = "user1".to_string();
        let user_value = User {
            id: "user1".to_string(),
            key: PRIVATE_KEY.to_string(),
        };

        (user_key, user_value)
    }

    #[tokio::test]
    async fn test_from_id() {
        let (user_key, user_value) = setup();
        let mut lock = get_sled_db().unwrap().read().unwrap();

        // Create a new user
        lock.borrow_mut()
            .create(user_key.clone(), user_value.clone())
            .unwrap();

        // Get the user by id
        let user_from_id = User::from_id(user_key.clone()).await.unwrap();

        // Check that the user data is correct
        assert_eq!(user_from_id.id, user_value.id);
        assert_eq!(user_from_id.key, user_value.key);

        // Should return an error if the user is not found
        assert!(User::from_id("user2".to_string()).await.is_err());
    }

    mod repository {
        use super::*;

        #[test]
        fn test_create() {
            let (user_key, user_value) = setup();
            let mut lock = get_sled_db().unwrap().read().unwrap();

            // Create a new user
            assert!(lock.borrow_mut().create(user_key, user_value).is_ok());
        }

        #[test]
        fn test_read() {
            let (user_key, user_value) = setup();
            let lock = get_sled_db().unwrap().read().unwrap();

            // Create a new user
            lock.create(user_key.clone(), user_value.clone()).unwrap();

            // Read the user
            let read_user: User = lock.read(user_key.clone()).unwrap().unwrap();

            // Check that the user data is correct
            assert_eq!(read_user.id, user_value.id);
            assert_eq!(read_user.key, user_value.key);
        }

        #[test]
        fn test_update() {
            let (user_key, mut user_value) = setup();
            let lock = get_sled_db().unwrap().read().unwrap();

            // Create a new user
            lock.create(user_key.clone(), user_value.clone()).unwrap();

            // Update the user data
            user_value.key = PRIVATE_KEY.to_string();

            // Update the user in the repository
            assert!(lock.update(user_key.clone(), user_value.clone()).is_ok());

            // Read the user
            let read_user: User = lock.read(user_key.clone()).unwrap().unwrap();

            // Check that the user data is updated
            assert_eq!(read_user.id, user_value.id);
            assert_eq!(read_user.key, user_value.key);
        }

        #[test]
        fn test_delete() {
            let (user_key, user_value) = setup();
            let lock = get_sled_db().unwrap().read().unwrap();

            // Create a new user
            lock.create(user_key.clone(), user_value.clone()).unwrap();

            // Delete the user
            let deleted_user: User = lock.delete(user_key.clone()).unwrap();

            // Check that the deleted user data is correct
            assert_eq!(deleted_user.id, user_value.id);
            assert_eq!(deleted_user.key, user_value.key);
        }
    }
}
