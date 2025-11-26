use crate::config::Config;
use crate::http_client::HttpClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct Payload {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Deserialize)]
struct Choice { message: MessageContent }

#[derive(Deserialize)]
struct MessageContent { content: String }

#[derive(Deserialize)]
struct ApiResponse { choices: Vec<Choice> }

pub struct AstraClient {
    cfg: Config,
    http: HttpClient,
}

impl AstraClient {
    pub fn new(cfg: Config, http: HttpClient) -> Self {
        Self { cfg, http }
    }

    pub async fn chat(&self, system: &str, user: &str) -> Result<String> {
        let payload = Payload {
            model: self.cfg.model.clone(),
            messages: vec![
                Message { role: "system".to_string(), content: system.to_string() },
                Message { role: "user".to_string(), content: user.to_string() },
            ],
            stream: false,
        };
        let resp: ApiResponse = self
            .http
            .post_json(&self.cfg.api_url, &payload, Some(&self.cfg.api_key))
            .await?;
        let content = resp
            .choices
            .get(0)
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "".to_string());
        Ok(content)
    }
}
