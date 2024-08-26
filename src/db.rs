use std::collections::HashMap;

pub struct Database {
    store: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }
    
    pub fn delete(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }
    
}

