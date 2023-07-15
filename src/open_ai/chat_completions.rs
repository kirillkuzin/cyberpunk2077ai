use crate::open_ai::core::{Chat, ChatCompletionParameters, ChatStorage, CHAT_STORAGE};
use crate::open_ai::settings::SETTINGS;

use std::{error::Error, vec};

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use tokio::runtime::Runtime;

pub fn get_last_answer_content(chat_id: String) -> String {
    let chat_storage: &ChatStorage = &CHAT_STORAGE.chat_storage;
    let chat: &Chat = chat_storage.get_or_create_chat(&chat_id);
    chat.get_last_answer_content()
}

fn wrapped_chat_completion_request(history: Vec<ChatCompletionParameters>) -> String {
    let rt = Runtime::new().unwrap();
    let _result = rt
        .block_on(chat_completion_request(history))
        .unwrap_or_else(|err| err.to_string());

    _result
}

pub fn schedule_chat_completion_request(chat_id: String, message: String) {
    std::thread::spawn(move || {
        let chat_storage: &ChatStorage = &CHAT_STORAGE.chat_storage;
        let chat: &Chat = chat_storage.get_or_create_chat(chat_id.as_str());
        let history = chat.get_history();

        let _result = wrapped_chat_completion_request(history);

        chat.add_answer(_result);
    });
}

async fn chat_completion_request(
    history: Vec<ChatCompletionParameters>,
) -> Result<String, Box<dyn Error>> {
    let settings = &SETTINGS.settings;

    let api_key = settings.get_api_key();
    let org_id = settings.get_org_id();

    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_org_id(org_id);
    let client = Client::with_config(config);

    // let messages = build_chat_completion_request_message_args(messages)?;
    let request = build_chat_completion_request(history)?;

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
    history: Vec<ChatCompletionParameters>,
) -> Result<CreateChatCompletionRequest, Box<dyn Error>> {
    let settings = &SETTINGS.settings;

    let model = settings.get_model();
    let max_tokens = settings.get_max_tokens();

    let mut chat_completion_request_message_args = vec![];
    for message in history {
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
