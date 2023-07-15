mod open_ai;

use red4ext_rs::prelude::*;

define_plugin! {
    name: "CyberAI",
    author: "author",
    version: 0:0:1,
    on_register: {
        register_function!("ScheduleChatCompletionRequest", open_ai::chat_completions::schedule_chat_completion_request);
        register_function!("GetLastAnswerContent", open_ai::chat_completions::get_last_answer_content);
        register_function!("GetSettings", open_ai::settings::get_settings);
    }
}
