// ScheduleChatCompletionRequest("your_custom_id", {{"System", "You are very helpful assistant."}, {"User", "How are you?"}});
native func ScheduleChatCompletionRequest(chat_id: String, messages: array<array<String>>);

// LogChannel("DEBUG", GetAnswer("your_custom_id"));
native func GetAnswer(chat_id: String) -> String;

// LogChannel("DEBUG", GetRequest("your_custom_id"));
native func GetRequest(chat_id: String) -> String;

// LogChannel("DEBUG", GetHistory("your_custom_id"));
native func GetHistory(chat_id: String) -> array<array<String>>;

// LogChannel("DEBUG", GetHistoryAsString("your_custom_id"));
native func GetHistoryAsString(chat_id: String) -> String;

// FlushChat("your_custom_id");
native func FlushChat(chat_id: String);

// LogChannel("DEBUG", GetSettings());
native func GetSettings() -> String;

// IterateHistory("your_custom_id");
public func IterateHistory(chat_id: String) {
    let history = GetHistory(chat_id);
    for completion in history {
        LogChannel(n"DEBUG", "Role: " + ToString(completion[0]) + "\nMessage: " + ToString(completion[1]));
    }
}