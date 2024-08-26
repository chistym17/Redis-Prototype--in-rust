use std::net::TcpListener;
use std::io::Read;
use std::io::Write;
pub async  fn start_server(){
    let listener=TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("Server running on 127.0.0.1:6379");

    loop {
        let (mut socket, _)=listener.accept().unwrap();
        tokio::spawn(async move{

            let mut buffer=vec![0;1024];
            loop {
                let n=socket.read(&mut buffer).unwrap();
                if n==0{
                    return ;
                }
                let response = String::from_utf8_lossy(&buffer[..n]);
                socket.write_all(response.as_bytes()).unwrap();
            }

        });
    }
}