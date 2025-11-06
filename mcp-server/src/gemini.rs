//! AI API client (Groq/Gemini).
//!
//! This module handles all communication with AI APIs (primarily Groq, with Gemini fallback),
//! including building requests, making HTTP calls, and parsing responses.

use crate::models::*;
use reqwest::Client;

/// Processes text through the Gemini API.
///
/// This function handles the complete flow of:
/// 1. Building the Gemini API request with system instructions
/// 2. Converting conversation history to Gemini format
/// 3. Making the HTTP request to Gemini
/// 4. Parsing the response and extracting the reply text
/// 5. Collecting metadata about token usage
///
/// # Arguments
///
/// * `client` - HTTP client for making requests
/// * `api_key` - Google Gemini API key for authentication
/// * `agent` - The agent configuration (model, system prompt)
/// * `user_text` - The user's input text
/// * `conversation_history` - Optional conversation history for context
///
/// # Returns
///
/// `Ok((reply_text, tokens_used))` on success, where:
/// - `reply_text` - The agent's response text
/// - `tokens_used` - Optional token count from Gemini
///
/// `Err(String)` on failure with error description
///
/// # Errors
///
/// Returns an error if:
/// - HTTP request fails
/// - Gemini API returns an error status
/// - Response parsing fails
/// - No candidate responses are returned
pub async fn process_with_gemini(
    client: &Client,
    api_key: &str,
    agent: &Agent,
    user_text: String,
    conversation_history: Option<Vec<Message>>,
    use_groq: bool,
) -> Result<(String, Option<u32>), String> {
    if use_groq {
        return process_with_groq(client, api_key, agent, user_text, conversation_history).await;
    }
    
    let mut contents = vec![];

    // Convert conversation history to Gemini format
    if let Some(history) = conversation_history {
        for msg in history {
            let role = match msg.role.as_str() {
                "user" => "user",
                "assistant" => "model",
                _ => continue,
            };
            contents.push(GeminiContent {
                role: role.to_string(),
                parts: vec![GeminiPart {
                    text: msg.content,
                }],
            });
        }
    }

    // Add the current user message
    contents.push(GeminiContent {
        role: "user".to_string(),
        parts: vec![GeminiPart { text: user_text }],
    });

    // Build the Gemini request with system instruction
    let gemini_request = GeminiRequest {
        contents,
        system_instruction: Some(GeminiSystemInstruction {
            parts: vec![GeminiPart {
                text: agent.system_prompt.clone(),
            }],
        }),
    };

    // Build the API URL
    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
        agent.model
    );

    // Make the HTTP request
    let response = client
        .post(&api_url)
        .header("x-goog-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&gemini_request)
        .send()
        .await
        .map_err(|e| format!("Gemini API request failed: {}", e))?;

    let response_status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read Gemini response: {}", e))?;

    // Check for HTTP errors
    if !response_status.is_success() {
        tracing::error!(
            "Gemini API error response ({}): {}",
            response_status,
            response_text
        );
        return Err(format!(
            "Gemini API error ({}): {}",
            response_status, response_text
        ));
    }

    tracing::info!("Gemini API response received successfully");

    // Parse the JSON response
    let gemini_response: GeminiResponse = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse Gemini response: {}. Raw: {}", e, response_text))?;

    // Extract the reply text
    let reply_text = gemini_response
        .candidates
        .first()
        .and_then(|c| c.content.parts.first())
        .map(|p| p.text.clone())
        .unwrap_or_else(|| "Sorry, I couldn't generate a response.".to_string());

    // Extract token usage metadata
    let tokens_used = gemini_response
        .usage_metadata
        .and_then(|u| u.total_token_count);

    Ok((reply_text, tokens_used))
}

/// Processes text through the Groq API (OpenAI-compatible).
async fn process_with_groq(
    client: &Client,
    api_key: &str,
    agent: &Agent,
    user_text: String,
    conversation_history: Option<Vec<Message>>,
) -> Result<(String, Option<u32>), String> {
    use serde_json::json;
    
    let mut messages = vec![
        json!({
            "role": "system",
            "content": agent.system_prompt.clone()
        })
    ];
    
    // Add conversation history if available
    if let Some(history) = conversation_history {
        for msg in history {
            messages.push(json!({
                "role": msg.role,
                "content": msg.content
            }));
        }
    }
    
    // Add current user message
    messages.push(json!({
        "role": "user",
        "content": user_text
    }));
    
    // Use a Groq-compatible model (llama models are fast and free)
    let groq_request = json!({
        "model": "llama-3.3-70b-versatile",  // Fast and capable model
        "messages": messages,
        "temperature": 0.7,
        "max_tokens": 1024
    });
    
    let api_url = "https://api.groq.com/openai/v1/chat/completions";
    
    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&groq_request)
        .send()
        .await
        .map_err(|e| format!("Groq API request failed: {}", e))?;
    
    let response_status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read Groq response: {}", e))?;
    
    if !response_status.is_success() {
        tracing::error!(
            "Groq API error response ({}): {}",
            response_status,
            response_text
        );
        return Err(format!(
            "Groq API error ({}): {}",
            response_status, response_text
        ));
    }
    
    tracing::info!("Groq API response received successfully");
    
    // Parse OpenAI-compatible response
    let groq_response: serde_json::Value = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse Groq response: {}. Raw: {}", e, response_text))?;
    
    // Extract the reply text from OpenAI-compatible format
    let reply_text = groq_response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Sorry, I couldn't generate a response.")
        .to_string();
    
    // Extract token usage
    let tokens_used = groq_response["usage"]["total_tokens"]
        .as_u64()
        .map(|t| t as u32);
    
    Ok((reply_text, tokens_used))
}
