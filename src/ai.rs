use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    role: String,
    content: String,
}

pub struct AiClient {
    api_key: Option<String>,
    model: String,
    messages: Vec<Message>,
}

impl AiClient {
    pub fn new() -> Self {
        Self {
            api_key: None,
            model: "gpt-3.5-turbo".to_string(),
            messages: Vec::new(),
        }
    }

    pub async fn send_message(&mut self, content: &str) -> Result<String> {
        // Add user message
        self.messages.push(Message {
            role: "user".to_string(),
            content: content.to_string(),
        });

        // TODO: Implement actual API calls to Ollama or OpenAI
        // For now, return a mock response
        let response =
            "I'm your AI assistant. I can help you with file operations and answer questions.";

        // Add assistant message
        self.messages.push(Message {
            role: "assistant".to_string(),
            content: response.to_string(),
        });

        Ok(response.to_string())
    }

    pub fn set_api_key(&mut self, key: String) {
        self.api_key = Some(key);
    }

    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }
}
