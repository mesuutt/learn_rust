use async_trait::async_trait;
use crate::user_client::{UserClient, ClientResp, ClientResult};

#[async_trait]
pub trait UserService {
    async fn login(&self, user_id: u64) -> ClientResult<ClientResp>;
}

pub struct UserServiceImpl {
    client: Box<dyn UserClient>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn login(&self, user_id: u64) -> ClientResult<ClientResp> {
        self.client.login(user_id).await
    }
}

pub fn new<T: 'static + UserClient>(client: T) -> impl UserService {
    UserServiceImpl {
        client: Box::new(client),
    }
}
