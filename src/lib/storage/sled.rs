use sled::Db;

use crate::core::repository::{Repository, RepositoryError};

/// Model is a trait that must be implemented by all models that are stored 
/// in the repository. This will allow blanket implementations of the 
/// repository for any struct that implements this trait.
pub trait SeldModel { }

pub struct SledRepository {
    db: Db,
}

impl SledRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl<M: SeldModel + AsRef<[u8]> + From<Vec<u8>>> Repository<M> for SledRepository {
    fn create(&self, key: String, value: M) -> Result<(), RepositoryError> {
        self.db.insert(key, value.as_ref()).map_err(|_| RepositoryError::InsertionError)?;
        Ok(())
    }

    fn read(&self, key: String) -> Result<Option<M>, RepositoryError> {
        let result = self.db.get(key).map_err(|_| RepositoryError::ReadError)?;
        Ok(result.map(|ivec| M::from(ivec.to_vec())))
    }

    fn update(&self, key: String, value: M) -> Result<(), RepositoryError> {
        self.db.insert(key, value.as_ref()).map_err(|_| RepositoryError::UpdateError)?;
        Ok(())
    }

    fn delete(&self, key: String) -> Result<(), RepositoryError> {
        self.db.remove(key).map_err(|_| RepositoryError::DeletionError)?;
        Ok(())
    }
}