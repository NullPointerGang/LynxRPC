use lynx_rpc::{server::Server, security::AuthValidator};

#[tokio::main]
async fn main() {
    // Устанавливаем секретный ключ
    std::env::set_var("LYNX_SECRET", "test_secret_key");
    
    let server = Server::bind("127.0.0.1:8080").await.unwrap();
    
    println!("Server started.");

    server.register_handler("add", |(a, b): (i32, i32)| {
        let result = a + b;
        Ok(result)
    });
    
    server.run().await;
}