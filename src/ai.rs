use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    role: String,
    content: String,
}

pub struct AiClient {
    api_key: Option<String>,
    model: String,
    messages: Vec<Message>,
    context: String,
}

impl AiClient {
    pub fn new() -> Self {
        Self {
            api_key: None,
            model: "gpt-3.5-turbo".to_string(),
            messages: Vec::new(),
            context: "You are fsbot-rs, an AI assistant that helps users with file operations and answers questions. \
                     You are friendly, helpful, and maintain context of the conversation. \
                     You can help with file searches, content analysis, and general questions.".to_string(),
        }
    }

    pub async fn send_message(&mut self, content: &str) -> Result<String> {
        // Add user message
        self.messages.push(Message {
            role: "user".to_string(),
            content: content.to_string(),
        });

        // Generate a response based on the conversation history and context
        let response = self.generate_response(content);

        // Add assistant message
        self.messages.push(Message {
            role: "assistant".to_string(),
            content: response.clone(),
        });

        Ok(response)
    }

    fn generate_response(&self, user_input: &str) -> String {
        // Simple pattern matching for now - we'll replace this with actual AI integration
        let input_lower = user_input.to_lowercase();

        // File-related queries
        if input_lower.contains("find") || input_lower.contains("search") {
            "I can help you search for files. Could you please specify what kind of files you're looking for? \
             For example, you can say 'find Python files' or 'search for documents'."
        } else if input_lower.contains("read") || input_lower.contains("open") {
            "I can help you read file contents. Please specify which file you'd like to read, \
             for example: 'read notes.txt' or 'open config.json'."
        } else if input_lower.contains("move") || input_lower.contains("organize") {
            "I can help you organize your files. Would you like me to help you move files to specific folders \
             or create a new organization structure?"
        } else if input_lower.contains("hello") || input_lower.contains("hi") {
            "Hello! I'm your AI assistant. I can help you with file operations and answer questions. \
             What would you like to do today?"
        } else if input_lower.contains("help") {
            "I can help you with various tasks:\n\
             - Search for files (e.g., 'find Python files')\n\
             - Read file contents (e.g., 'read notes.txt')\n\
             - Organize files (e.g., 'move files to folders')\n\
             - Answer questions about your files\n\
             What would you like to do?"
        } else if input_lower.contains("bye") || input_lower.contains("goodbye") {
            "Goodbye! Feel free to ask for help anytime. Have a great day!"
        } else {
            "I understand you're asking about something. Could you please provide more details? \
             I can help you with file operations, answer questions, or assist with other tasks."
        }.to_string()
    }

    pub fn set_api_key(&mut self, key: String) {
        self.api_key = Some(key);
    }

    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }
}
