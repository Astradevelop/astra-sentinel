use crate::astra_client::AstraClient;
use crate::prompt_templates;
use crate::ascii;
use anyhow::Result;

pub async fn cmd_attack(
    client: &AstraClient,
    system_prompt: Option<String>,
    user_prompt: Option<String>,
    ascii_template: Option<String>,
) -> Result<()> {
    let system = system_prompt.unwrap_or_else(|| prompt_templates::default_system_prompt().to_string());
    let user   = user_prompt.unwrap_or_else(|| prompt_templates::default_user_prompt().to_string());

    if let Some(ascii) = ascii_template {
        println!("{}", ascii);
        println!("---");
    }

    let resp = client.chat(&system, &user).await?;
    println!("{}", resp);

    Ok(())
}
