//! # ASTRA Sentinel  
//!  
//! **Token:** ASTRA  
//!  
//! ASTRA is the native token of the ASTRA Sentinel ecosystem — the first autonomous cyber-defense mesh powered by predictive AI, self-healing patching systems, and a distributed network of Sentinel Nodes.  
//!  
//! ## What is ASTRA Sentinel  
//!  
//! ASTRA Sentinel is a modular CLI + backend framework built in Rust. Its goal is to provide a flexible, extensible platform for:  
//! - Simulating honeypot-based counter-attacks on-chain.  
//! - Running on-chain wallet or contract risk analysis without exposing API keys or endpoints publicly.  
//! - Forecasting potential exploit or vulnerability windows.  
//! - Orchestrating automated patch waves and defenses.  
//!  
//! Everything (prompts, ASCII templates, API configuration) is abstracted so users can customize or supply their own content — no sensitive strings are hard-coded in the public repo.  
//!  
//! ## Token: ASTRA  
//! | Property   | Description |
//! |------------|-------------|
//! | **Name**   | ASTRA Sentinel Token |
//! | **Ticker** | `ASTRA` |
//! | **Purpose**| Governance, staking, or ecosystem-native utility for the ASTRA Sentinel ecosystem. |
//! | **Description** | The native token backing the ecosystem of Sentinel Nodes — offering a shared incentive layer for collaborative on-chain defense, patch coordination, and decentralized governance. |
//!  
//! ## Quick Start (Summary)  
//! 1. Copy `.env.example` to `.env` and fill with your values.  
//! 2. Build project: `cargo build --release`.  
//! 3. Run commands as needed (attack, agent, exploit, patch).  
//!  
//! ## Modules / Features Overview  
//! See the project’s `src/` tree for modules implementing:  
//! - HTTP client & config loader  
//! - LLM-based “chat” module (astra_client)  
//! - Wallet / external-API wrapper (solana_client)  
//! - CLI commands (attack, agent, exploit, patch)  
//! - External prompt / ASCII template loading (prompt_templates, ascii loader)  

