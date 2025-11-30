use crate::Result;

/// A database storing key/value pairs.
///
/// TODO: consider using ByteView from Fjall for cheap byte slices.
#[derive(Default)]
pub struct Database {}

impl Database {
    /// Creates a new database.
    pub fn new() -> Self {
        Self::default()
    }

    /// Deletes a key. Ignores unknown keys.
    pub fn delete(&self, key: impl AsRef<[u8]>) -> Result<()> {
        Ok(())
    }

    /// Gets the value for a key. Returns None if the key does not exist.
    pub fn get(&self, key: impl AsRef<[u8]>) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }

    /// Sets a key to a value, replacing any existing value.
    pub fn set(&self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<()> {
        Ok(())
    }
}

/// TODO: use e.g. goldenscript for tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_set_get() -> Result<()> {
        let db = Database::new();

        db.set(b"key", b"value")?;
        assert_eq!(db.get(b"key")?, Some(b"value".to_vec()));
        assert_eq!(db.get(b"foo")?, None);

        Ok(())
    }
}
