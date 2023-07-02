// LogChannel("DEBUG", ChatCompletionRequest({{"System", "You are very helpful assistant."}, {"User", "How are you?"}}));
native func ChatCompletionRequest(messages: array<array<String>>) -> String;

// ScheduleChatCompletionRequest({{"System", "You are very helpful assistant."}, {"User", "How are you?"}});
native func ScheduleChatCompletionRequest(messages: array<array<String>>);

// LogChannel("DEBUG", GetLastAnswerContent());
native func GetLastAnswerContent() -> String;