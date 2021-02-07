use async_trait::async_trait;
use serde::Deserialize;

pub type ClientResult<T> = Result<T, reqwest::Error>;

#[derive(Debug, Deserialize)]
pub struct ClientResp {
    pub success: bool,
}

#[async_trait]
pub trait UserClient {
    async fn login(&self, user_id: u64) -> ClientResult<ClientResp>;
}

pub struct UserClientImpl;

#[async_trait]
impl UserClient for UserClientImpl {
    async fn login(&self, _user_id: u64) -> ClientResult<ClientResp> {
        let resp = reqwest::get("https://reqbin.com/echo/get/json").await?;
        Ok(resp.json::<ClientResp>().await?)
    }
}

pub fn new() -> impl UserClient {
    UserClientImpl
}
