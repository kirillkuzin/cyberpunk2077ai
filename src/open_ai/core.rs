use std::collections::HashMap;

use async_openai::types::Role;
use lazy_static::lazy_static;
use crossbeam_queue::SegQueue;


pub struct ChatCompletionParameters {
    pub role: Role,
    pub content: String,
}

impl Into<ChatCompletionParameters> for Vec<String> {
    fn into(self) -> ChatCompletionParameters {
        if self.len() != 2 {
            panic!("Invalid vector length");
        }

        let role = match self[0].as_str() {
            "System" => Role::System,
            "User" => Role::User,
            "Assistant" => Role::Assistant,
            _ => panic!("Invalid role value"),
        };

        ChatCompletionParameters {
            role,
            content: self[1].clone(),
        }
    }
}

impl Clone for ChatCompletionParameters {
    fn clone(&self) -> ChatCompletionParameters {
        ChatCompletionParameters {
            role: self.role.clone(),
            content: self.content.clone(),
        }
    }
}


pub struct Chat {
    pub id: String,
    answers: SegQueue<String>,
    history: Vec<ChatCompletionParameters>,
}

impl Chat {
    fn new() -> Self {
        Chat {
            id: "default".to_string(), // TODO: generate unique id for each chat
            answers: SegQueue::new(),
            history: Vec::new(),
        }
    }

    pub fn get_last_answer_content(&self) -> String {
        self.answers.pop().unwrap_or_default()
    }

    pub fn add_answer(&self, answer: String) {
        self.answers.push(answer);
    }

    pub fn add_history(&mut self, role: Role, message: String) {
        let history_entry = ChatCompletionParameters {
            role,
            content: message,
        };
        self.history.push(history_entry);
    }

    pub fn get_history(&self) -> Vec<ChatCompletionParameters> {
        self.history.clone()
    }
}


pub struct ChatStorage {
    chats: HashMap<String, Chat>,
}

impl ChatStorage {
    pub fn new() -> Self {
        ChatStorage {
            chats: HashMap::new(),
        }
    }

    pub fn get_or_create_chat(&mut self, chat_id: &str) -> &Chat {
        self.chats.entry(chat_id.to_string()).or_insert(Chat::new())
    }
}


struct GlobalChatStorage {
    pub chat_storage: ChatStorage,
}

lazy_static! {
    pub static ref CHAT_STORAGE: GlobalChatStorage = GlobalChatStorage {
        chat_storage: ChatStorage::new(),
    };
}
