use crate::models::MintResult;
use anyhow::{anyhow, Result};
use reqwest::Client;
use std::env;
use uuid::Uuid;

/// Mint a token on-chain (or mock). Returns tx hash and optional token id.
pub async fn mint_token(metadata_url: &str, recipient: &str) -> Result<MintResult> {
    if let Ok(rpc) = env::var("BLOCKCHAIN_RPC") {
        tracing::info!(rpc = %rpc, "calling configured blockchain RPC");
        let client = Client::new();
        // For simplicity we POST a JSON body {metadata_url, recipient}
        let body = serde_json::json!({"metadata_url": metadata_url, "recipient": recipient});
        let resp = client
            .post(&rpc)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!("rpc request failed: {}", e))?;

        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(anyhow!("rpc call failed: {} - {}", status, text));
        }

        // Attempt to parse a tx hash and token id
        let (tx_hash, token_id) = match resp.json::<serde_json::Value>().await {
            Ok(json) => (
                json.get("tx_hash").and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or(text.clone()),
                json.get("token_id").and_then(|v| v.as_str().map(|s| s.to_string())),
            ),
            Err(_) => (text.clone(), None),
        };

        Ok(MintResult { tx_hash, token_id })
    } else {
        // Mock path
    let tx_hash = format!("0x{}", Uuid::new_v4().to_simple());
    let token_id = Some(format!("{}", Uuid::new_v4().to_simple()));
        tracing::warn!(tx_hash = %tx_hash, "BLOCKCHAIN_RPC not set - returning mock mint result");
        Ok(MintResult { tx_hash, token_id })
    }
}
