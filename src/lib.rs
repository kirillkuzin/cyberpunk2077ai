use std::env;
use std::fs;
use std::path::PathBuf;
use std::{error::Error, vec};

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};
use crossbeam_queue::SegQueue;
use lazy_static::lazy_static;
use red4ext_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

define_plugin! {
    name: "CyberAI",
    author: "author",
    version: 0:0:1,
    on_register: {
        register_function!("ChatCompletionRequest", wrapped_chat_completion_request);
        register_function!("ScheduleChatCompletionRequest", schedule_chat_completion_request);
        register_function!("GetLastAnswerContent", get_last_answer_content);
        register_function!("GetSettings", get_settings);
    }
}

#[derive(Deserialize, Serialize)]
struct Settings {
    api_key: String,
    org_id: String,
    model: String,
    max_tokens: u16,
}

impl Into<String> for &Settings {
    fn into(self) -> String {
        serde_json::to_string(self).expect("Failed to serialize JSON")
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            api_key: String::new(),
            org_id: String::new(),
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: 512u16,
        }
    }
}

struct GlobalSettings {
    settings: Settings,
}

lazy_static! {
    static ref SETTINGS: GlobalSettings = {
        let exe_path: PathBuf = env::current_exe().unwrap();
        let settings_path: PathBuf = exe_path
            .ancestors()
            .nth(3)
            .unwrap()
            .join("red4ext/plugins/CyberAI/Settings.json");

        let mut settings: Settings = Settings::default();

        match fs::read_to_string(settings_path) {
            Ok(contents) => {
                settings = serde_json::from_str(&contents).unwrap();
            }
            Err(err) => {
                eprintln!("Failed to read file: {}\nDefault settings will be use", err);
            }
        }

        GlobalSettings { settings }
    };
}

struct ChatCompletionParameters {
    role: Role,
    content: String,
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

static ANSWERS: SegQueue<String> = SegQueue::new();

fn get_settings() -> String {
    let settings: &Settings = &SETTINGS.settings;

    settings.into()
}

fn get_last_answer_content() -> String {
    let value = ANSWERS.pop().unwrap_or_default();

    value
}

fn wrapped_chat_completion_request(messages: Vec<Vec<String>>) -> String {
    let rt = Runtime::new().unwrap();
    let _result = rt
        .block_on(chat_completion_request(messages))
        .unwrap_or_else(|err| err.to_string());

    _result
}

fn schedule_chat_completion_request(messages: Vec<Vec<String>>) {
    std::thread::spawn(move || {
        let _result = wrapped_chat_completion_request(messages);
        ANSWERS.push(_result);
    });
}

async fn chat_completion_request(messages: Vec<Vec<String>>) -> Result<String, Box<dyn Error>> {
    let settings: &Settings = &SETTINGS.settings;

    let api_key = settings.api_key.clone();
    let org_id = settings.org_id.clone();

    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_org_id(org_id);
    let client = Client::with_config(config);
    let messages = build_chat_completion_request_message_args(messages)?;
    let request = build_chat_completion_request(messages)?;
    let response = client.chat().create(request).await?;

    let choice = response.choices[0]
        .message
        .content
        .clone()
        .unwrap_or_default();

    Ok(choice)
}

fn build_chat_completion_request_message_args(
    messages: Vec<Vec<String>>,
) -> Result<Vec<ChatCompletionParameters>, Box<dyn Error>> {
    let mut chat_completion_request_message_args = vec![];
    for message in messages {
        let chat_completion_params: ChatCompletionParameters = message.into();
        chat_completion_request_message_args.push(chat_completion_params);
    }

    Ok(chat_completion_request_message_args)
}

fn build_chat_completion_request(
    messages: Vec<ChatCompletionParameters>,
) -> Result<CreateChatCompletionRequest, Box<dyn Error>> {
    let settings: &Settings = &SETTINGS.settings;

    let model = settings.model.clone();
    let max_tokens = settings.max_tokens.clone();

    let mut chat_completion_request_message_args = vec![];
    for message in messages {
        chat_completion_request_message_args.push(
            ChatCompletionRequestMessageArgs::default()
                .role(message.role)
                .content(message.content)
                .build()?,
        );
    }

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(max_tokens)
        .model(model)
        .messages(chat_completion_request_message_args)
        .build()?;

    Ok(request)
}
