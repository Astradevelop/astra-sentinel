use anyhow::{Context, Result};
use std::env;

pub struct Config {
    pub api_url: String,
    pub api_key: String,
    pub model: String,
    pub solana_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            api_url: env::var("ASTRA_API_URL").context("ASTRA_API_URL not set")?,
            api_key: env::var("ASTRA_API_KEY").context("ASTRA_API_KEY not set")?,
            model: env::var("ASTRA_MODEL").context("ASTRA_MODEL not set")?,
            solana_key: env::var("SOLANA_API_KEY").context("SOLANA_API_KEY not set")?,
        })
    }
}
