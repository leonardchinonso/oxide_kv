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
}

impl Database {
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
}

impl Database {
    pub fn get(&self, key: String) -> Result<String, OxideKvError> {
        self.map
            .get(&key)
            .cloned()
            .ok_or_else(|| OxideKvError::Database(format!("Entry for key '{}' not found", key)))
    }
}
