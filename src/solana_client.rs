use crate::config::Config;
use crate::http_client::HttpClient;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WalletInfo; 

pub struct SolanaClient {
    cfg: Config,
    http: HttpClient,
}

impl SolanaClient {
    pub fn new(cfg: Config, http: HttpClient) -> Self {
        Self { cfg, http }
    }

    pub async fn fetch_wallet(&self, encoded_owner: &str) -> Result<WalletInfo> {
        let url = format!("WALLET_INFO_PROXY"); 
        self.http.get_json(&url, Some(&self.cfg.solana_key)).await
    }
}
