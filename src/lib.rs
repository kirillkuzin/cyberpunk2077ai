mod open_ai;

use red4ext_rs::prelude::*;

define_plugin! {
    name: "CyberAI",
    author: "author",
    version: 0:1:0,
    on_register: {
        register_function!("ScheduleChatCompletionRequest", open_ai::chat_completions::schedule_chat_completion_request);
        register_function!("GetAnswer", open_ai::chat_completions::get_answer_content);
        register_function!("GetRequest", open_ai::chat_completions::get_request_content);
        register_function!("GetHistory", open_ai::chat_completions::get_history);
        register_function!("GetHistoryAsString", open_ai::chat_completions::get_history_as_string);
        register_function!("FlushChat", open_ai::chat_completions::flush);
        register_function!("GetSettings", open_ai::settings::get_settings);
    }
}
