use tokio::io::{AsyncWriteExt, AsyncReadExt};


#[tokio::test]
async fn test_server(){

    let mut socket=tokio::net::TcpStream::connect("127.0.0.1:6380").await.unwrap();
    
    socket.write(b"SET key1 value1\n").await.unwrap();
    let mut buf=vec![0;1024];
    let n=socket.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..n], b"ok\n");


    socket.write(b"GET key1\n").await.unwrap();
    let n = socket.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..n], b"value1\n");


}

  