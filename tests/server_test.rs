// use std::net::TcpListener;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use redis_prototype::db::Database;
// use redis_prototype::server::start_server;
// use std::time::Duration;

#[tokio::test]
async fn test_server(){

    // let db=Arc::new(Database::new());

    // let listener = TcpListener::bind("127.0.0.1:6381").unwrap();
    // let db_clone = db.clone();
    // tokio::spawn(async move {
    //     start_server().await;
    // });
    //client connect--testing the set command

    let mut socket=tokio::net::TcpStream::connect("127.0.0.1:6380").await.unwrap();
    socket.write(b"SET key1 value1\n").await.unwrap();
    let mut buf=vec![0;1024];
    let n=socket.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..n], b"ok\n");

    //testing the get command

    socket.write(b"GET key1\n").await.unwrap();
    let n = socket.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..n], b"value1\n");


}

  