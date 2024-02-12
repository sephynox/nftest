use std::sync::RwLock;

use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use sled::Db;

use crate::core::repository::{Repository, RepositoryError};

/// Model is a trait that must be implemented by all models that are stored
/// in the repository. This will allow blanket implementations of the
/// repository for any struct that implements this trait.
pub trait SledModel: serde::Serialize + DeserializeOwned {
    /// Convert the model to a vector of bytes.
    fn to_vec(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    /// Convert a vector of bytes to a model.
    fn from_vec(v: Vec<u8>) -> Self {
        // TODO: Handle errors
        bincode::deserialize(&v).unwrap()
    }
}

pub struct SledRepository {
    db: Db,
}

impl SledRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl<M: SledModel> Repository<M> for SledRepository {
    fn create(&self, key: String, value: M) -> Result<(), RepositoryError> {
        self.db
            .insert(key, value.to_vec())
            .map_err(|_| RepositoryError::InsertionError)?;
        Ok(())
    }

    fn read(&self, key: String) -> Result<Option<M>, RepositoryError> {
        let result = self.db.get(key).map_err(|_| RepositoryError::ReadError)?;
        Ok(result.map(|v| M::from_vec(v.to_vec())))
    }

    fn update(&self, key: String, value: M) -> Result<(), RepositoryError> {
        self.db
            .insert(key, value.to_vec())
            .map_err(|_| RepositoryError::UpdateError)?;
        Ok(())
    }

    fn delete(&self, key: String) -> Result<M, RepositoryError> {
        let result = self
            .db
            .remove(key)
            .map_err(|_| RepositoryError::DeletionError)?;
        if let Some(result) = result {
            Ok(M::from_vec(result.to_vec()))
        } else {
            Err(RepositoryError::DeletionError)
        }
    }
}

lazy_static! {
    static ref REPO: RwLock<SledRepository> = {
        let db_url = match cfg!(test) {
            // For tests, use a temporary database
            true => "/tmp/test_sled_db".to_string(),
            // For production, use the database URL
            false => std::env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set")),
        };
        let config = sled::Config::new().path(db_url);
        let db = config.open().unwrap_or_else(|_| panic!("Failed to open database"));

        RwLock::new(SledRepository::new(db))
    };
}

pub fn get_sled_db() -> Result<&'static RwLock<SledRepository>, RepositoryError> {
    Ok(&*REPO)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use sled::Config;

    #[derive(Clone, Serialize, Deserialize)]
    struct TestModel {
        data: String,
    }

    impl SledModel for TestModel {}

    #[test]
    fn test_create_read_update_delete() {
        let config = Config::new().temporary(true);
        let db = config.open().unwrap();
        let repo = SledRepository::new(db);

        let key = "test".to_string();
        let value = TestModel {
            data: "test data".to_string(),
        };

        // Test create
        assert!(repo.create(key.clone(), value.clone()).is_ok());

        // Test read
        let read_value: TestModel = repo.read(key.clone()).unwrap().unwrap();
        assert_eq!(read_value.data, value.data);

        // Test update
        let updated_value = TestModel {
            data: "updated data".to_string(),
        };
        assert!(repo.update(key.clone(), updated_value.clone()).is_ok());

        // Test read after update
        let read_value: TestModel = repo.read(key.clone()).unwrap().unwrap();
        assert_eq!(read_value.data, updated_value.data);

        // Test delete
        let deleted_value: TestModel = repo.delete(key.clone()).unwrap();
        assert_eq!(deleted_value.data, updated_value.data);
    }

    #[test]
    fn test_get_sled_db() -> Result<(), Box<dyn std::error::Error>> {
        // Call the function
        let _db = get_sled_db()?;

        // If we got here, the function didn't return an error
        Ok(())
    }
}
