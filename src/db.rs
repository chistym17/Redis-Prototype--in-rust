use std::collections::{HashMap,HashSet,VecDeque};
use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::{Write, Read};

#[derive(Serialize, Deserialize)]
pub enum Value {
    String(String),
    List(VecDeque<String>),
    Set(HashSet<String>),
}
#[derive(Serialize, Deserialize)]
pub struct Database {
    store: HashMap<String, Value>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            store: HashMap::new(),
        }
    }

    pub fn save_snaps(&self,file_path:&str)->std::io::Result<()>{
        let data=bincode::serialize(self).unwrap();
        let mut file=File::create(file_path)?;
        file.write_all(&data)?;
        Ok(())
    }

    pub fn load_snaps(file_path:&str)->std::io::Result<Self>{
        let mut file=File::open(file_path)?;
        let mut buffer=Vec::new();
        file.read_to_end(&mut buffer)?;
        let db=bincode::deserialize(&buffer).unwrap();
        Ok(db)
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

    pub fn rpush(&mut self,key: String,value: String)->usize{
        let entry=self.store.entry(key).or_insert_with(||Value::List(VecDeque::new()));
        if let Value::List(ref mut list)=entry{
            list.push_back(value);
            list.len()
        }
        else {
            {
                0
            }
        }
    }

    pub fn lpop(&mut self, key: &str) -> Option<String> {
        if let Some(Value::List(ref mut list)) = self.store.get_mut(key) {
            list.pop_front()
        } else {
            None
        }
    }

    pub fn rpop(&mut self,key: &str)->Option<String>
    {
        if let Some(Value::List(ref mut list))=self.store.get_mut(key){
            list.pop_back()

        }else{
            None
        }
    }
    
   // SET 
    pub fn sadd(&mut self, key: String, value: String) -> bool {
        let entry = self.store.entry(key).or_insert_with(|| Value::Set(HashSet::new()));
        if let Value::Set(ref mut set) = entry {
            set.insert(value)
        } else {
            false 
        }
    }

    pub fn srem(&mut self, key: String, value: &str) -> bool {
        if let Some(Value::Set(ref mut set)) = self.store.get_mut(&key) {
            set.remove(value)
        } else {
            false
        }
    }

    pub fn smembers(&self, key: &str) -> Option<&HashSet<String>> {
        if let Some(Value::Set(ref set)) = self.store.get(key) {
            Some(set)
        } else {
            None
        }
    }
    
}

