use crate::astra_client::AstraClient;
use crate::solana_client::SolanaClient;
use anyhow::Result;

pub async fn cmd_agent(
    client: &AstraClient,
    sol: &SolanaClient,
    wallet_address: &str,
    user_question: Option<String>,
) -> Result<()> {
    let _ = sol.fetch_wallet(wallet_address).await?;

    let question = user_question.unwrap_or_default();
    let resp = client.chat("", &question).await?;
    println!("{}", resp);

    Ok(())
}
