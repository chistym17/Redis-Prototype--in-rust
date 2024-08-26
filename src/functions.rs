fn handle_command(db:&Arc<Mutex<Database>>,command:&[u8])->String
{
    let str=String::from_utf8_lossy(command);
    let parts=str.split_whitespace().collect();
    match parts.as_slice()
    {
        ["SET",key,value]=>{
            let mut db=db.lock().unwrap();
            db.set(key.to_string(),value.to_string());
            "ok\n".to_string()
        }

        ["GET",key]=>{
            let db=db.lock().unwrap();
            match db.get(key){
                Some(value) => format!("{}\n", value),
                None => "nil\n".to_string(),
            }
        }

        ["DEL", key] => {
            let mut db = db.lock().unwrap();
            if db.del(key) {
                "1\n".to_string()
            } else {
                "0\n".to_string()
            }
        }

        _ =>"ERR unknown command\n".to_string(),
        


    }

}