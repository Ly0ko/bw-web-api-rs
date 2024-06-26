use reqwest::Client;
use crate::error::ApiError;

#[derive(Clone)]
pub struct BWClient {
    client: Client,
    base_url: String,
}

impl BWClient {
    pub fn new(base_url: String) -> Result<Self, ApiError> {
        Ok(Self {
            client: Client::new(),
            base_url,
        })
    }

    pub async fn fetch(&self, path: &str) -> Result<String, ApiError> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self.client.get(&url).send().await?.text().await?;
        Ok(response)
    }
}
