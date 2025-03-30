use lynx_rpc::{client::Client, security::AuthValidator};

#[tokio::main]
async fn main() {
    // Устанавливаем секретный ключ
    std::env::set_var("LYNX_SECRET", "test_secret_key");
    
    let mut client = Client::connect("127.0.0.1:8080").await.unwrap();
    
    // Генерируем токен
    let validator = AuthValidator::new();
    let token = validator.generate_token();
    
    match client.call::<_, i32>("add", &token, (2, 3)).await {
        Ok(result) => println!("Результат: {}", result),
        Err(e) => eprintln!("Ошибка: {:?}", e),
    }
}