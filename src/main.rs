mod db;
mod server;
mod functions;


#[tokio::main]
async fn main() {
    println!("Starting the redis server...");
    server::start_server().await;
}
