use crate::open_ai::core::{
    append_answer_to_chat, append_request_to_chat, append_to_chat_history, flush_chat,
    get_answer_from_chat, get_full_chat_history, get_iterable_chat_history,
    get_printable_chat_history, get_request_from_chat, ChatCompletionParameters,
};
use crate::open_ai::settings::SETTINGS;

use std::{error::Error, vec};

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};
use tokio::runtime::Runtime;

pub fn get_answer_content(chat_id: String) -> String {
    get_answer_from_chat(&chat_id)
}

pub fn get_request_content(chat_id: String) -> String {
    get_request_from_chat(&chat_id)
}

pub fn get_history(chat_id: String) -> Vec<Vec<String>> {
    get_iterable_chat_history(&chat_id)
}

pub fn get_history_as_string(chat_id: String) -> String {
    get_printable_chat_history(&chat_id)
}

pub fn flush(chat_id: String) {
    flush_chat(&chat_id);
}

fn wrapped_chat_completion_request(history: Vec<ChatCompletionParameters>) -> String {
    let rt = Runtime::new().unwrap();
    let _result = rt
        .block_on(chat_completion_request(history))
        .unwrap_or_else(|err| err.to_string());

    _result
}

pub fn schedule_chat_completion_request(chat_id: String, messages: Vec<Vec<String>>) {
    std::thread::spawn(move || {
        for message in messages.iter() {
            append_request_to_chat(&chat_id, message[1].clone());

            let role: Role = match message[0].as_str() {
                "User" => Role::User,
                "System" => Role::System,
                "Assistant" => Role::Assistant,
                _ => Role::User,
            };
            append_to_chat_history(&chat_id, role, message[1].clone());
        }

        let history = get_full_chat_history(&chat_id);

        let _result: String = wrapped_chat_completion_request(history);

        append_answer_to_chat(&chat_id, _result.clone());
        append_to_chat_history(&chat_id, Role::Assistant, _result.clone());
    });
}

async fn chat_completion_request(
    messages: Vec<ChatCompletionParameters>,
) -> Result<String, Box<dyn Error>> {
    let settings = &SETTINGS.settings;

    let api_key = settings.get_api_key();
    let org_id = settings.get_org_id();

    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_org_id(org_id);
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

fn build_chat_completion_request(
    messages: Vec<ChatCompletionParameters>,
) -> Result<CreateChatCompletionRequest, Box<dyn Error>> {
    let settings = &SETTINGS.settings;

    let model = settings.get_model();
    let max_tokens = settings.get_max_tokens();

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
