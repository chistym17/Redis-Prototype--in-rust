use std::collections::{hash_map, HashMap};

pub struct Database{
 store:HashMap<String,String>,
}

impl Database {
    pub fn new()->self{
        Database{
            store:HashMap::new(),
        }
    }
}

pub fn set(&mut self,key:String,value:String)
{
    self.store.insert(key,value);
}

pub fn get(&self,key:&str)->Option<&String>{
    self.store.get(key)
}

pub fn delete(&mut self,key:&str)->Option<&String>{
    self.store.remove(key)
}