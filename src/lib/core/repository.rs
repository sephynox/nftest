use thiserror::Error;

/// RepositoryError is an enum that contains all the possible errors that 
/// can occur when using a repository.
#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Failed to insert record")]
    InsertionError,
    #[error("Failed to read record")]
    ReadError,
    #[error("Failed to update record")]
    UpdateError,
    #[error("Failed to delete record")]
    DeletionError,
}

/// Repository is a trait that must be implemented by all repositories.
pub trait Repository<M> {
    /// Create a new record in the repository.
    fn create(&self, key: String, value: M) -> Result<(), RepositoryError>;
    /// Read a record from the repository.
    fn read(&self, key: String) -> Result<Option<M>, RepositoryError>;
    /// Update a record in the repository.
    fn update(&self, key: String, value: M) -> Result<(), RepositoryError>;
    /// Delete a record from the repository.
    fn delete(&self, key: String) -> Result<(), RepositoryError>;
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, marker::PhantomData, sync::RwLock};

    use serde::{de::DeserializeOwned, Deserialize, Serialize};

    use super::*;

    /// HashMapRepository is a simple in-memory repository that uses a 
    /// HashMap to store the data.
    pub struct HashMapRepository<M: Serialize + DeserializeOwned> {
        _marker: PhantomData<M>,
        map: RwLock<HashMap<String, Vec<u8>>>,
    }
    
    impl<M: Serialize + DeserializeOwned> HashMapRepository<M> {
        pub fn new() -> Self {
            Self {
                _marker: PhantomData,
                map: RwLock::new(HashMap::new()),
            }
        }
    }
    
    impl<M: Serialize + DeserializeOwned> Repository<M> for HashMapRepository<M> {
        fn create(&self, key: String, value: M) -> Result<(), RepositoryError> {
            let mut map = self.map.write().unwrap();
            map.insert(key, serde_json::to_vec(&value).unwrap());
            Ok(())
        }
    
        fn read(&self, key: String) -> Result<Option<M>, RepositoryError> {
            let map = self.map.read().unwrap();
            match map.get(&key) {
                Some(value) => Ok(Some(serde_json::from_slice(value).unwrap())),
                None => Ok(None),
            }
        }
    
        fn update(&self, key: String, value: M) -> Result<(), RepositoryError> {
            let mut map = self.map.write().unwrap();
            map.insert(key, serde_json::to_vec(&value).unwrap());
            Ok(())
        }
    
        fn delete(&self, key: String) -> Result<(), RepositoryError> {
            let mut map = self.map.write().unwrap();
            map.remove(&key);
            Ok(())
        }
    }

    #[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
    struct TestModel(Vec<u8>);

    #[test]
    fn test_repository() {
        let repo: HashMapRepository<TestModel> = HashMapRepository::new();
        let key = "test_key".to_string();
        let value = TestModel(vec![4, 5, 6]);

        // Test create
        assert!(repo.create(key.clone(), value.clone()).is_ok());

        // Test read
        assert_eq!(repo.read(key.clone()).unwrap(), Some(value.clone()));

        // Test update
        let new_value = TestModel(vec![7, 8, 9]);
        assert!(repo.update(key.clone(), new_value.clone()).is_ok());
        assert_eq!(repo.read(key.clone()).unwrap(), Some(new_value.clone()));

        // Test delete
        assert!(repo.delete(key.clone()).is_ok());
        assert_eq!(repo.read(key.clone()).unwrap(), None);
    }
}