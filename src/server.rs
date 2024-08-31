use crate::functions::{handle_command,take_snaps,load_latest_snap};
use std::net::TcpListener;
use std::io::Read;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub async  fn start_server(){

    let snapshot_file_path = "db_snapshot.bin".to_string();
    let db = Arc::new(Mutex::new(load_latest_snap(&snapshot_file_path)));

    let snapshot_interval = Duration::from_secs(60); 
    take_snaps(db.clone(), snapshot_interval, snapshot_file_path);
   

    let listener=TcpListener::bind("127.0.0.1:6380").unwrap();
    println!("Server running on 127.0.0.1:6380");

    loop {
        let db = db.clone();
        let (mut socket, _)=listener.accept().unwrap();
        tokio::spawn(async move{

            let mut buffer=vec![0;1024];
            loop {
                let n=socket.read(&mut buffer).unwrap();
                if n==0{
                    return ;
                }
                let response = handle_command(&db, &buffer[..n]);
                socket.write_all(response.as_bytes()).unwrap();
            }

        });
    }
}