use serde::{Deserialize, Serialize};

/// Request payload sent by front-end to trigger a mint.
#[derive(Debug, Deserialize)]
pub struct MintRequest {
    /// Human-friendly name/title
    pub name: String,
    /// Description or transcript
    pub description: Option<String>,
    /// Link to uploaded asset (image/audio) on your storage (optional)
    pub asset_url: Option<String>,
    /// Recipient address for token (optional; can be assigned server-side)
    pub recipient: Option<String>,
}

/// Internal metadata object that will be uploaded to storage (IPFS etc.)
#[derive(Debug, Serialize)]
pub struct Metadata {
    pub name: String,
    pub description: Option<String>,
    pub asset_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResult {
    /// Content identifier (CID) or equivalent from storage
    pub cid: String,
    /// A full gateway URL to retrieve the metadata
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MintResult {
    /// Blockchain transaction hash
    pub tx_hash: String,
    /// Token ID minted (if available)
    pub token_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MintResponse {
    pub status: String,
    pub upload: UploadResult,
    pub mint: MintResult,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
