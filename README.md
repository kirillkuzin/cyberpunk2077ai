# CyberAI

Welcome to the CyberAI project! This plugin for Cyberpunk 2077 enables integration between the videogame and OpenAI API, opening a world of possibilities for enhancing the gameplay experience. With this plugin, you can call OpenAI API methods directly from the scripting level.

Plugin development is still in progress.

![bg](bg.png)

## Installation
1. Download the zip from the latest release
2. Move CyberAI.dll and Settings.json to red4ext\plugins\CyberAI
3. Open Settings.json and paste your OpenAI API key and organization id
4. If you would access to the plugin's functions from CET console, you need to move CyberAI.reds to r6\scripts\CyberAI

## Usage

### Redscript

Ask GPT to generate an answer for you:
```
native func ScheduleChatCompletionRequest(chat_id: String, messages: array<array<String>>);

ScheduleChatCompletionRequest("your_custom_id", {{"System", "Speak as ..."}, {"User", "How are you?"}});

ScheduleChatCompletionRequest("your_custom_id", {{"User", "My name is V"}, {"User", "How are you and what's my name?"}});
```

You can collect your request and an answer when it is done:
```
native func GetAnswer(chat_id: String) -> String;
native func GetRequest(chat_id: String) -> String;

LogChannel(n"DEBUG", GetAnswer("your_custom_id"));
LogChannel(n"DEBUG", GetRequest("your_custom_id"));
```

You can iterate through the chat history, or you can get it as a string:
```
native func GetHistory(chat_id: String) -> array<array<String>>;
native func GetHistoryAsString(chat_id: String) -> String;

LogChannel(n"DEBUG", GetHistoryAsString("your_custom_id"));

let history = GetHistory(chat_id);
for completion in history {
    LogChannel(n"DEBUG", "Role: " + ToString(completion[0]) + "\nMessage: " + ToString(completion[1]));
}
```

Flushing a chat:
```
native func FlushChat(chat_id: String);

FlushChat("your_custom_id");
```

You need to put your generated custom string ID in almost all functions. CyberAI will provide a new chat for every unique ID and keep the chat history. 

<b>Whenever you request to chat, all the chat history also sends to OpenAI API.</b>

## Inspiration

With this plugin, the possibilities are almost limitless. Here are just a few examples of how you can use it:

- **AI-NPC Dialogue:** Use OpenAI's GPT to generate unique dialogue for non-player characters (NPCs), increasing the diversity and richness of in-game interactions.

- **Dynamic Plot Generation:** Use OpenAI to generate unique storylines or side quests based on in-game events or player actions.

- **Procedural Mission Planning:** Generate procedural missions based on context, NPC data, and player preferences using AI.

- **Interactive Environment**: Use AI to generate dynamic responses from the environment, making your exploration of Night City even more immersive.

- **Intelligent Enemy Tactics:** AI could control enemy tactics based on the player's strategy and actions, making combat more challenging and unpredictable.

Remember, these are just examples, and the only limit is your imagination!

## Dependencies

This project would not be possible without the following dependencies:

- [red4ext-rs](https://github.com/jac3km4/red4ext-rs): A Rust binding for RED4ext, which provides an API to interact with the internals of Cyberpunk 2077.

- [async-openai](https://github.com/64bit/async-openai): An asynchronous, modern Rust library for OpenAI API.

- [OpenAI API](https://openai.com/blog/openai-api): OpenAI offers a general-purpose "text in, text out" interface, making it possible to use language models like GPT-3 in a wide range of applications.

## License

This project is licensed under the terms of the MIT License. See the LICENSE file in the project's root directory for more details.

Enjoy exploring the new world of Cyberpunk 2077 with the power of AI!

---
