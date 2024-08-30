use crate::db::Database;
use crate::functions::handle_command;

use std::net::TcpListener;
use std::io::Read;
use std::io::Write;
use std::sync::{Arc, Mutex};

pub async  fn start_server(){

   

    let listener=TcpListener::bind("127.0.0.1:6380").unwrap();
    println!("Server running on 127.0.0.1:6380");

    loop {
        let db = Arc::new(Mutex::new(Database::new()));
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