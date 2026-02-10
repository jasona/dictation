use super::*;

// ---- Request serialization tests ----

#[test]
fn openai_request_serializes_correctly() {
    let request = OpenAiRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            OpenAiMessage {
                role: "system".to_string(),
                content: "System prompt".to_string(),
            },
            OpenAiMessage {
                role: "user".to_string(),
                content: "Hello world".to_string(),
            },
        ],
        temperature: 0.1,
        max_tokens: 2048,
    };

    let json = serde_json::to_value(&request).unwrap();
    assert_eq!(json["model"], "gpt-4o-mini");
    assert_eq!(json["messages"].as_array().unwrap().len(), 2);
    assert_eq!(json["messages"][0]["role"], "system");
    assert_eq!(json["messages"][1]["role"], "user");
    assert_eq!(json["messages"][1]["content"], "Hello world");
    let temp = json["temperature"].as_f64().unwrap();
    assert!((temp - 0.1).abs() < 0.001, "temperature was {}", temp);
    assert_eq!(json["max_tokens"], 2048);
}

#[test]
fn anthropic_request_serializes_correctly() {
    let request = AnthropicRequest {
        model: "claude-haiku-4-5-20251001".to_string(),
        max_tokens: 2048,
        system: "System prompt".to_string(),
        messages: vec![AnthropicMessage {
            role: "user".to_string(),
            content: "Hello world".to_string(),
        }],
    };

    let json = serde_json::to_value(&request).unwrap();
    assert_eq!(json["model"], "claude-haiku-4-5-20251001");
    assert_eq!(json["max_tokens"], 2048);
    assert_eq!(json["system"], "System prompt");
    assert_eq!(json["messages"].as_array().unwrap().len(), 1);
    assert_eq!(json["messages"][0]["role"], "user");
    assert_eq!(json["messages"][0]["content"], "Hello world");
}

// ---- Response deserialization tests ----

#[test]
fn openai_response_deserializes() {
    let json = r#"{
        "choices": [
            {
                "message": {
                    "content": "Cleaned text here"
                }
            }
        ]
    }"#;

    let response: OpenAiResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.choices.len(), 1);
    assert_eq!(response.choices[0].message.content, "Cleaned text here");
}

#[test]
fn anthropic_response_deserializes() {
    let json = r#"{
        "content": [
            {
                "type": "text",
                "text": "Cleaned text here"
            }
        ]
    }"#;

    let response: AnthropicResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.content.len(), 1);
    assert_eq!(response.content[0].text, "Cleaned text here");
}

#[test]
fn openai_response_empty_choices() {
    let json = r#"{"choices": []}"#;
    let response: OpenAiResponse = serde_json::from_str(json).unwrap();
    assert!(response.choices.is_empty());
}

#[test]
fn anthropic_response_empty_content() {
    let json = r#"{"content": []}"#;
    let response: AnthropicResponse = serde_json::from_str(json).unwrap();
    assert!(response.content.is_empty());
}

// ---- API key management tests ----
// These tests interact with the system keyring, which may require
// platform-specific permissions. They use unique names to avoid collisions.

#[test]
fn api_key_crud_openai() {
    let provider = CloudProvider::OpenAi;
    let test_key = "sk-test-openai-key-12345";

    // Save
    let save_result = save_api_key(&provider, test_key);
    if save_result.is_err() {
        eprintln!("Skipping keyring test (not available): {}", save_result.unwrap_err());
        return;
    }

    // Get — verify round-trip
    match get_api_key(&provider) {
        Ok(retrieved) => {
            assert_eq!(retrieved, test_key);
            assert!(has_api_key(&provider));

            // Delete
            delete_api_key(&provider).unwrap();
            assert!(!has_api_key(&provider));
        }
        Err(e) => {
            eprintln!("Skipping keyring test (get failed after save): {}", e);
        }
    }
}

#[test]
fn api_key_crud_anthropic() {
    let provider = CloudProvider::Anthropic;
    let test_key = "sk-ant-test-key-12345";

    let save_result = save_api_key(&provider, test_key);
    if save_result.is_err() {
        eprintln!("Skipping keyring test (not available): {}", save_result.unwrap_err());
        return;
    }

    match get_api_key(&provider) {
        Ok(retrieved) => {
            assert_eq!(retrieved, test_key);
            assert!(has_api_key(&provider));

            delete_api_key(&provider).unwrap();
            assert!(!has_api_key(&provider));
        }
        Err(e) => {
            eprintln!("Skipping keyring test (get failed after save): {}", e);
        }
    }
}

// ---- Live API tests (require real keys) ----

#[test]
#[ignore]
fn live_openai_cleanup() {
    let client = reqwest::blocking::Client::new();
    let provider = CloudProvider::OpenAi;

    assert!(has_api_key(&provider), "Set OpenAI API key first");

    let cleaner = CloudCleaner::new(&client, provider);
    let result = cleaner.clean("um so basically i went to the store").unwrap();
    assert!(!result.is_empty());
    assert!(!result.contains("um"));
}

#[test]
#[ignore]
fn live_anthropic_cleanup() {
    let client = reqwest::blocking::Client::new();
    let provider = CloudProvider::Anthropic;

    assert!(has_api_key(&provider), "Set Anthropic API key first");

    let cleaner = CloudCleaner::new(&client, provider);
    let result = cleaner.clean("um so basically i went to the store").unwrap();
    assert!(!result.is_empty());
    assert!(!result.contains("um"));
}
