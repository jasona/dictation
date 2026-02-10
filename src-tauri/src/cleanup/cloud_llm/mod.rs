// Cloud LLM cleanup (OpenAI, Anthropic)

use super::{CloudProvider, TextCleaner};
use serde::{Deserialize, Serialize};

const CLEANUP_PROMPT: &str = "Clean up the following dictated text. Fix grammar and punctuation. Remove filler words. Do NOT change technical terms, names, or meaning. Do NOT add content. Return only the cleaned text.";

// ---- API key management via keyring ----

fn keyring_service() -> &'static str {
    "vozr"
}

fn key_name(provider: &CloudProvider) -> &'static str {
    match provider {
        CloudProvider::OpenAi => "openai_api_key",
        CloudProvider::Anthropic => "anthropic_api_key",
    }
}

pub fn save_api_key(provider: &CloudProvider, key: &str) -> Result<(), String> {
    let entry = keyring::Entry::new(keyring_service(), key_name(provider))
        .map_err(|e| format!("Keyring error: {}", e))?;
    entry
        .set_password(key)
        .map_err(|e| format!("Failed to save API key: {}", e))
}

pub fn get_api_key(provider: &CloudProvider) -> Result<String, String> {
    let entry = keyring::Entry::new(keyring_service(), key_name(provider))
        .map_err(|e| format!("Keyring error: {}", e))?;
    entry
        .get_password()
        .map_err(|e| format!("API key not found: {}", e))
}

pub fn has_api_key(provider: &CloudProvider) -> bool {
    get_api_key(provider).is_ok()
}

pub fn delete_api_key(provider: &CloudProvider) -> Result<(), String> {
    let entry = keyring::Entry::new(keyring_service(), key_name(provider))
        .map_err(|e| format!("Keyring error: {}", e))?;
    entry
        .delete_credential()
        .map_err(|e| format!("Failed to delete API key: {}", e))
}

// ---- OpenAI request/response types ----

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiResponseMessage,
}

#[derive(Deserialize)]
struct OpenAiResponseMessage {
    content: String,
}

// ---- Anthropic request/response types ----

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Deserialize)]
struct AnthropicContent {
    text: String,
}

// ---- Cloud cleaner ----

pub struct CloudCleaner<'a> {
    client: &'a reqwest::blocking::Client,
    provider: CloudProvider,
}

impl<'a> CloudCleaner<'a> {
    pub fn new(client: &'a reqwest::blocking::Client, provider: CloudProvider) -> Self {
        Self { client, provider }
    }

    fn clean_openai(&self, text: &str, api_key: &str) -> Result<String, String> {
        let request = OpenAiRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![
                OpenAiMessage {
                    role: "system".to_string(),
                    content: CLEANUP_PROMPT.to_string(),
                },
                OpenAiMessage {
                    role: "user".to_string(),
                    content: text.to_string(),
                },
            ],
            temperature: 0.1,
            max_tokens: 2048,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request)
            .send()
            .map_err(|e| format!("OpenAI request failed: {}", e))?;

        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err("Invalid OpenAI API key".to_string());
        }
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err("OpenAI rate limit exceeded".to_string());
        }
        if !status.is_success() {
            return Err(format!("OpenAI API error: HTTP {}", status));
        }

        let body: OpenAiResponse = response
            .json()
            .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

        body.choices
            .into_iter()
            .next()
            .map(|c| c.message.content.trim().to_string())
            .ok_or_else(|| "OpenAI returned no choices".to_string())
    }

    fn clean_anthropic(&self, text: &str, api_key: &str) -> Result<String, String> {
        let request = AnthropicRequest {
            model: "claude-haiku-4-5-20251001".to_string(),
            max_tokens: 2048,
            system: CLEANUP_PROMPT.to_string(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: text.to_string(),
            }],
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| format!("Anthropic request failed: {}", e))?;

        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err("Invalid Anthropic API key".to_string());
        }
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err("Anthropic rate limit exceeded".to_string());
        }
        if !status.is_success() {
            return Err(format!("Anthropic API error: HTTP {}", status));
        }

        let body: AnthropicResponse = response
            .json()
            .map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;

        body.content
            .into_iter()
            .next()
            .map(|c| c.text.trim().to_string())
            .ok_or_else(|| "Anthropic returned no content".to_string())
    }
}

impl TextCleaner for CloudCleaner<'_> {
    fn clean(&self, text: &str) -> Result<String, String> {
        let api_key = get_api_key(&self.provider)?;

        match self.provider {
            CloudProvider::OpenAi => self.clean_openai(text, &api_key),
            CloudProvider::Anthropic => self.clean_anthropic(text, &api_key),
        }
    }
}

/// Test an API key by sending a trivial request.
pub fn test_cloud_key(
    client: &reqwest::blocking::Client,
    provider: &CloudProvider,
) -> Result<(), String> {
    let api_key = get_api_key(provider)?;

    let cleaner = CloudCleaner::new(client, provider.clone());
    let result = match *provider {
        CloudProvider::OpenAi => cleaner.clean_openai("Hello", &api_key),
        CloudProvider::Anthropic => cleaner.clean_anthropic("Hello", &api_key),
    };

    result.map(|_| ())
}

#[cfg(test)]
mod tests;
