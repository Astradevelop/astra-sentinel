mod config;
mod http_client;
mod astra_client;
mod solana_client;
mod prompt_templates;
mod ascii;
mod commands;

use clap::{Parser, Subcommand};
use config::Config;
use http_client::HttpClient;
use astra_client::AstraClient;
use solana_client::SolanaClient;
use dotenvy::dotenv;

#[derive(Parser)]
#[command(name = "astra-sentinel", about = "ASTRA SENTINEL CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Attack {
        #[arg(long)]
        sys_prompt_file: Option<std::path::PathBuf>,
        #[arg(long)]
        user_prompt: Option<String>,
        #[arg(long)]
        ascii_file: Option<std::path::PathBuf>,
    },
    Agent {
        #[arg(long)]
        wallet: String,
        #[arg(long)]
        question: Option<String>,
    },
    Exploit {
        #[arg(long)]
        sys_prompt_file: Option<std::path::PathBuf>,
        #[arg(long)]
        user_prompt: Option<String>,
    },
    Patch {
        #[arg(long)]
        sys_prompt_file: Option<std::path::PathBuf>,
        #[arg(long)]
        user_prompt: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cfg = Config::from_env()?;
    let http = HttpClient::new();
    let client = AstraClient::new(cfg.clone(), http.clone());
    let sol = SolanaClient::new(cfg.clone(), http);

    let cli = Cli::parse();
    match cli.command {
        Commands::Attack { sys_prompt_file, user_prompt, ascii_file } => {
            let sys = sys_prompt_file.and_then(|p| std::fs::read_to_string(&p).ok());
            let ascii = ascii_file.and_then(|p| std::fs::read_to_string(&p).ok());
            commands::attack::cmd_attack(&client, sys, user_prompt, ascii).await?;
        }
        Commands::Agent { wallet, question } => {
            commands::agent::cmd_agent(&client, &sol, &wallet, question).await?;
        }
        Commands::Exploit { sys_prompt_file, user_prompt } => {
            let sys = sys_prompt_file.and_then(|p| std::fs::read_to_string(&p).ok());
            commands::exploit::cmd_exploit(&client, sys, user_prompt).await?;
        }
        Commands::Patch { sys_prompt_file, user_prompt } => {
            let sys = sys_prompt_file.and_then(|p| std::fs::read_to_string(&p).ok());
            commands::patch::cmd_patch(&client, sys, user_prompt).await?;
        }
    }

    Ok(())
}
