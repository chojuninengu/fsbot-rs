use crate::filesystem::FileSystem;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    filesystem: FileSystem,
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
            filesystem: FileSystem::new(),
        }
    }

    pub async fn send_message(&mut self, content: &str) -> Result<String> {
        // Add user message
        self.messages.push(Message {
            role: "user".to_string(),
            content: content.to_string(),
        });

        // Generate a response based on the conversation history and context
        let response = self.process_command(content).await?;

        // Add assistant message
        self.messages.push(Message {
            role: "assistant".to_string(),
            content: response.clone(),
        });

        Ok(response)
    }

    async fn process_command(&mut self, user_input: &str) -> Result<String> {
        let input_lower = user_input.to_lowercase();

        // File creation
        if input_lower.contains("create") && input_lower.contains("file") {
            let file_name = extract_filename(user_input);
            if let Some(name) = file_name {
                match self.filesystem.create_file(&name) {
                    Ok(_) => Ok(format!("I've created the file: {}", name)),
                    Err(e) => Ok(format!("Error creating file: {}", e)),
                }
            } else {
                Ok(
                    "Please specify a filename to create. Example: 'create file notes.txt'"
                        .to_string(),
                )
            }
        }
        // File deletion
        else if input_lower.contains("delete") || input_lower.contains("remove") {
            let file_name = extract_filename(user_input);
            if let Some(name) = file_name {
                match self.filesystem.delete_file(&name) {
                    Ok(_) => Ok(format!("I've deleted the file: {}", name)),
                    Err(e) => Ok(format!("Error deleting file: {}", e)),
                }
            } else {
                Ok("Please specify a filename to delete. Example: 'delete notes.txt'".to_string())
            }
        }
        // File search
        else if input_lower.contains("find") || input_lower.contains("search") {
            let query = extract_search_query(user_input);
            match self.filesystem.search_files(&query) {
                Ok(files) => {
                    if files.is_empty() {
                        Ok("No files found matching your search.".to_string())
                    } else {
                        let file_list = files
                            .iter()
                            .map(|f| f.to_string_lossy().to_string())
                            .collect::<Vec<String>>()
                            .join("\n");
                        Ok(format!("Found these files:\n{}", file_list))
                    }
                }
                Err(e) => Ok(format!("Error searching files: {}", e)),
            }
        }
        // File reading
        else if input_lower.contains("read") || input_lower.contains("open") {
            let file_name = extract_filename(user_input);
            if let Some(name) = file_name {
                match self.filesystem.read_file(&PathBuf::from(name)) {
                    Ok(content) => Ok(format!("File contents:\n{}", content)),
                    Err(e) => Ok(format!("Error reading file: {}", e)),
                }
            } else {
                Ok("Please specify a filename to read. Example: 'read notes.txt'".to_string())
            }
        }
        // Help command
        else if input_lower.contains("help") {
            Ok("I can help you with these file operations:\n\
                - Create files: 'create file notes.txt'\n\
                - Delete files: 'delete notes.txt'\n\
                - Search files: 'find Python files' or 'search documents'\n\
                - Read files: 'read notes.txt'\n\
                What would you like to do?"
                .to_string())
        }
        // Greetings
        else if input_lower.contains("hello") || input_lower.contains("hi") {
            Ok("Hello! I'm your AI assistant. I can help you with file operations and answer questions. \
                What would you like to do today?".to_string())
        }
        // Goodbye
        else if input_lower.contains("bye") || input_lower.contains("goodbye") {
            Ok("Goodbye! Feel free to ask for help anytime. Have a great day!".to_string())
        }
        // Default response
        else {
            Ok("I understand you're asking about something. Could you please provide more details? \
                I can help you with file operations, answer questions, or assist with other tasks.".to_string())
        }
    }

    pub fn set_api_key(&mut self, key: String) {
        self.api_key = Some(key);
    }

    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }
}

fn extract_filename(input: &str) -> Option<String> {
    let words: Vec<&str> = input.split_whitespace().collect();
    for (i, word) in words.iter().enumerate() {
        if word.contains('.') && i > 0 {
            return Some(word.to_string());
        }
    }
    None
}

fn extract_search_query(input: &str) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut query = String::new();
    for (i, word) in words.iter().enumerate() {
        if i > 0 && !word.eq_ignore_ascii_case("files") {
            query.push_str(word);
            query.push(' ');
        }
    }
    query.trim().to_string()
}
