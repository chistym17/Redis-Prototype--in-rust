use std::sync::{Arc, Mutex};
use crate::db::Database;
use crate::db::Value;
use std::thread;
use std::time::Duration;

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
        //List commands
        ["LPUSH", key, value] => {
            let mut db = db.lock().unwrap();
            let len = db.lpush(key.to_string(), value.to_string());
            format!("{}\n", len)
        }
        
        ["RPUSH", key, value] => {
            let mut db = db.lock().unwrap();
            let len = db.rpush(key.to_string(), value.to_string());
            format!("{}\n", len)
        }
        
        ["LPOP", key] => {
            let mut db = db.lock().unwrap();
            match db.lpop(key) {
                Some(value) => format!("{}\n", value),
                None => "nil\n".to_string(),
            }
        }
        
        ["RPOP", key] => {
            let mut db = db.lock().unwrap();
            match db.rpop(key) {
                Some(value) => format!("{}\n", value),
                None => "nil\n".to_string(),
            }
        }
        ////////////
        
        ["SADD", key, value] => {
            let mut db = db.lock().unwrap();
            let added = db.sadd(key.to_string(), value.to_string());
            format!("{}\n", if added { "1" } else { "0" })
        }
        
        ["SREM", key, value] => {
            let mut db = db.lock().unwrap();
            let removed = db.srem(key.to_string(), value);
            format!("{}\n", if removed { "1" } else { "0" })
        }
        
        ["SMEMBERS", key] => {
            let db = db.lock().unwrap();
            match db.smembers(key) {
                Some(set) => set.iter().cloned().collect::<Vec<String>>().join(" ") + "\n",
                None => "nil\n".to_string(),
            }
        }
        
        

        _ =>"ERR unknown command\n".to_string(),
        


    }

}

pub fn take_snaps(db:Arc<Mutex<Database>>,interval:Duration,file_path:String)
{
    thread::spawn(move || {
        loop{
            thread::sleep(interval);
            let datab=db.lock().unwrap();
            datab.save_snaps(&file_path).expect("failed snap");
            println!("Snapshot saved!");
        }
    });
}

fn load_latest_snapshot() -> Database {
    match Database::load_snaps("db_snapshot.bin") {
        Ok(db) => db,
        Err(_) => {
            println!("No snapshot found, starting with an empty database.");
            Database::new()
        }
    }
}
