use std::sync::{Arc, Mutex};
use crate::db::Database;
use crate::db::Value;

pub fn handle_command(db:&Arc<Mutex<Database>>,command:&[u8])->String
{
    let str=String::from_utf8_lossy(command);
    let parts:Vec<&str>=str.split_whitespace().collect();
    match parts.as_slice()
    {
        ["SET", key, value] => {
            let mut db = db.lock().unwrap();
            db.set(key.to_string(), Value::String(value.to_string())); // Use Value::String
            "ok\n".to_string()
        }
        

        ["GET", key] => {
            let db = db.lock().unwrap();
            match db.get(key) {
                Some(Value::String(value)) => format!("{}\n", value), // Handle Value::String
                Some(_) => "ERR wrong type\n".to_string(), // If the value is not a string
                None => "nil\n".to_string(),
            }
        }
        

        ["DEL", key] => {
            let mut db = db.lock().unwrap();
            if db.delete(key) {
                "1\n".to_string()
            } else {
                "0\n".to_string()
            }
        }

        _ =>"ERR unknown command\n".to_string(),
        


    }

}