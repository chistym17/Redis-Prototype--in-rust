use std::collections::{HashMap,HashSet,VecDeque};

pub enum Value {
    String(String),
    List(VecDeque<String>),
    Set(HashSet<String>),
    Hash(HashMap<String, String>),
}
pub struct Database {
    store: HashMap<String, Value>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.store.get(key)
    }
    
    pub fn delete(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }

    pub fn lpush(&mut self, key: String, value: String) -> usize {
        let entry = self.store.entry(key).or_insert_with(|| Value::List(VecDeque::new()));
        if let Value::List(ref mut list) = entry {
            list.push_front(value);
            list.len()
        } else {
            0 
        }
    }
    
}

