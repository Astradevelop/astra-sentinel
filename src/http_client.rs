use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    pub async fn get_json<T: DeserializeOwned>(&self, url: &str, api_key: Option<&str>) -> Result<T> {
        let mut req = self.client.get(url);
        if let Some(k) = api_key {
            req = req.header("x-api-key", k);
        }
        let res = req.send().await?;
        if !res.status().is_success() {
            return Err(anyhow!("GET {} failed: {}", url, res.status()));
        }
        Ok(res.json::<T>().await?)
    }

    pub async fn post_json<B: Serialize, T: DeserializeOwned>(
        &self,
        url: &str,
        body: &B,
        api_key: Option<&str>,
    ) -> Result<T> {
        let mut req = self.client.post(url).json(body);
        if let Some(k) = api_key {
            req = req.header("x-api-key", k);
        }
        let res = req.send().await?;
        if !res.status().is_success() {
            let txt = res.text().await.unwrap_or_default();
            return Err(anyhow!("POST {} failed: {} — {}", url, res.status(), txt));
        }
        Ok(res.json::<T>().await?)
    }
}
