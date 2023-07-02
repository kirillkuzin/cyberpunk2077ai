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
use red4ext_rs::prelude::*;
use tokio::runtime::Runtime;

define_plugin! {
    name: "CyberAI",
    author: "author",
    version: 0:0:1,
    on_register: {
        register_function!("ChatCompletionRequest", wrapped_chat_completion_request);
        register_function!("ScheduleChatCompletionRequest", schedule_chat_completion_request);
        register_function!("GetLastAnswerContent", get_last_answer_content);
    }
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
    let api_key = "";
    let org_id = "";

    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_org_id(org_id);
    let messages = build_chat_completion_request_message_args(messages)?;
    let client = Client::with_config(config);
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
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages(chat_completion_request_message_args)
        .build()?;

    Ok(request)
}
