mod user_service;
mod user_client;
use user_service::UserService;


#[tokio::main]
async fn main() {
    let client = user_client::new();
    let user_service =  user_service::new(client);
    let result = user_service.login(1);
    println!("result: {:?}", result.await);
}