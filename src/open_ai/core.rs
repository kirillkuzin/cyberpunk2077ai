use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use async_openai::types::Role;
use crossbeam_queue::SegQueue;
use lazy_static::lazy_static;

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

impl Into<Vec<String>> for ChatCompletionParameters {
    fn into(self) -> Vec<String> {
        let role = match self.role {
            Role::System => "System",
            Role::User => "User",
            Role::Assistant => "Assistant",
            Role::Function => "Function",
        };

        vec![role.to_string(), self.content]
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

struct Chat {
    answers: SegQueue<String>,
    requests: SegQueue<String>,
    history: Vec<ChatCompletionParameters>,
}

impl Chat {
    fn new() -> Self {
        Chat {
            answers: SegQueue::new(),
            requests: SegQueue::new(),
            history: Vec::new(),
        }
    }

    fn add_request(&self, request: String) {
        self.requests.push(request);
    }

    fn get_request_content(&self) -> String {
        self.requests.pop().unwrap_or_default()
    }

    fn add_answer(&self, answer: String) {
        self.answers.push(answer);
    }

    fn get_answer_content(&self) -> String {
        self.answers.pop().unwrap_or_default()
    }

    fn add_history(&mut self, role: Role, message: String) {
        let history_entry = ChatCompletionParameters {
            role,
            content: message,
        };
        self.history.push(history_entry);
    }

    fn get_history(&self) -> Vec<ChatCompletionParameters> {
        self.history.clone()
    }

    fn flush(&mut self) {
        self.answers = SegQueue::new();
        self.requests = SegQueue::new();
        self.history.clear();
    }
}

struct ChatStorage {
    chats: HashMap<String, Chat>,
}

impl ChatStorage {
    fn new() -> Self {
        ChatStorage {
            chats: HashMap::new(),
        }
    }

    fn get_or_create_chat(&mut self, chat_id: &str) -> &mut Chat {
        self.chats.entry(chat_id.to_string()).or_insert(Chat::new())
    }
}

struct GlobalChatStorage {
    chat_storage: ChatStorage,
}

lazy_static! {
    static ref CHAT_STORAGE: Mutex<GlobalChatStorage> = Mutex::new(GlobalChatStorage {
        chat_storage: ChatStorage::new(),
    });
}

fn with_chat<F, R>(chat_id: &str, f: F) -> R
where
    F: FnOnce(&mut Chat) -> R,
{
    let mut chat_storage_guard: MutexGuard<GlobalChatStorage> = CHAT_STORAGE.lock().unwrap();
    let chat_storage = &mut chat_storage_guard.chat_storage;
    let chat = chat_storage.get_or_create_chat(chat_id);

    f(chat)
}

pub fn append_request_to_chat(chat_id: &str, request: String) {
    with_chat(chat_id, |chat| chat.add_request(request));
}

pub fn get_request_from_chat(chat_id: &str) -> String {
    with_chat(chat_id, |chat| chat.get_request_content())
}

pub fn append_answer_to_chat(chat_id: &str, answer: String) {
    with_chat(chat_id, |chat| chat.add_answer(answer));
}

pub fn get_answer_from_chat(chat_id: &str) -> String {
    with_chat(chat_id, |chat| chat.get_answer_content())
}

pub fn append_to_chat_history(chat_id: &str, role: Role, message: String) {
    with_chat(chat_id, |chat| chat.add_history(role, message));
}

pub fn get_full_chat_history(chat_id: &str) -> Vec<ChatCompletionParameters> {
    with_chat(chat_id, |chat| chat.get_history())
}

pub fn get_iterable_chat_history(chat_id: &str) -> Vec<Vec<String>> {
    let history = get_full_chat_history(chat_id);
    let mut result = vec![];
    for entry in history {
        result.push(entry.into());
    }
    result
}

pub fn get_printable_chat_history(chat_id: &str) -> String {
    let history = get_full_chat_history(chat_id);
    let mut result = String::new();
    for entry in history {
        result.push_str(&format!("{}: {}\n", entry.role, entry.content));
    }
    result
}

pub fn flush_chat(chat_id: &str) {
    with_chat(chat_id, |chat| chat.flush());
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_openai::types::Role;

    #[test]
    fn test_append_and_get_request() {
        let chat_id = "test_chat";
        let request = "Hello, world!";
        append_request_to_chat(chat_id, request.to_string());

        let result = get_request_from_chat(chat_id);
        assert_eq!(result, request);
    }

    #[test]
    fn test_append_and_get_answer() {
        let chat_id = "test_chat";
        let answer = "Hello, user!";
        append_answer_to_chat(chat_id, answer.to_string());

        let result = get_answer_from_chat(chat_id);
        assert_eq!(result, answer);
    }

    #[test]
    fn test_append_and_get_chat_history() {
        let chat_id = "test_chat";
        let request = "Hello, world!";
        let answer = "Hello, user!";
        append_to_chat_history(chat_id, Role::User, request.to_string());
        append_to_chat_history(chat_id, Role::Assistant, answer.to_string());

        let history = get_full_chat_history(chat_id);
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].role, Role::User);
        assert_eq!(history[0].content, request);
        assert_eq!(history[1].role, Role::Assistant);
        assert_eq!(history[1].content, answer);
    }

    #[test]
    fn test_get_iterable_chat_history() {
        let chat_id = "test_chat_iterable";
        let request = "Hello, world!";
        let answer = "Hello, user!";
        append_to_chat_history(chat_id, Role::User, request.to_string());
        append_to_chat_history(chat_id, Role::Assistant, answer.to_string());

        let history = get_iterable_chat_history(chat_id);
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], vec!["User", request]);
        assert_eq!(history[1], vec!["Assistant", answer]);
    }

    #[test]
    fn test_get_printable_chat_history() {
        let chat_id = "test_chat_printable";
        let request = "Hello, world!";
        let answer = "Hello, user!";
        append_to_chat_history(chat_id, Role::User, request.to_string());
        append_to_chat_history(chat_id, Role::Assistant, answer.to_string());

        let history = get_printable_chat_history(chat_id);
        let expected = format!(
            "{}: {}\n{}: {}\n",
            Role::User,
            request,
            Role::Assistant,
            answer
        );
        assert_eq!(history, expected);
    }

    #[test]
    fn test_flush_chat() {
        let chat_id = "test_chat_flush";
        let request = "Hello, world!";
        let answer = "Hello, user!";
        append_to_chat_history(chat_id, Role::User, request.to_string());
        append_to_chat_history(chat_id, Role::Assistant, answer.to_string());

        flush_chat(chat_id);
        let history = get_full_chat_history(chat_id);
        assert!(history.is_empty());

        let request_content = get_request_from_chat(chat_id);
        assert!(request_content.is_empty());

        let answer_content = get_answer_from_chat(chat_id);
        assert!(answer_content.is_empty());
    }
}
