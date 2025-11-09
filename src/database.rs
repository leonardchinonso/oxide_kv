use crate::error::OxideKvError;

pub struct Database {
    map: std::collections::HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }

    pub fn upsert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: String) -> Result<String, OxideKvError> {
        self.map
            .get(&key)
            .cloned()
            .ok_or_else(|| OxideKvError::Database(format!("Entry for key '{}' not found", key)))
    }

    pub fn remove(&mut self, key: String) -> Result<(), OxideKvError> {
        match self.map.contains_key(&key) {
            true => {
                self.map.remove(&key);
                Ok(())
            }
            false => Err(OxideKvError::Database(format!(
                "Entry for key '{}' not found",
                key
            ))),
        }
    }
}
