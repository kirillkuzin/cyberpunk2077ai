// ScheduleChatCompletionRequest("your_custom_id", {{"System", "You are very helpful assistant."}, {"User", "How are you?"}});
native func ScheduleChatCompletionRequest(chat_id: String, messages: array<array<String>>);

// LogChannel("DEBUG", GetAnswer("your_custom_id"));
native func GetAnswer(chat_id: String) -> String;

// LogChannel("DEBUG", GetRequest("your_custom_id"));
native func GetRequest(chat_id: String) -> String;

// LogChannel("DEBUG", GetHistory("your_custom_id"));
native func GetHistory(chat_id: String) -> String;

// LogChannel("DEBUG", GetSettings());
native func GetSettings() -> String;