# CyberAI

Welcome to the CyberAI project! This plugin for Cyberpunk 2077 enables integration between the videogame and OpenAI API, opening a world of possibilities for enhancing the gameplay experience. With this plugin, you can call OpenAI API methods directly from the scripting level.

Plugin development is still in progress.

![bg](bg.png)

## Installation

1. Download a zip from the latest release
2. Move CyberAI.dll and Settings.json to red4ext\plugins\CyberAI
3. Open Settings.json and paste your OpenAI api key and organization id

## Usage

### Redscript

```
native func ChatCompletionRequest(messages: array<array<String>>) -> String;
native func ScheduleChatCompletionRequest(messages: array<array<String>>);
native func GetLastAnswerContent() -> String;
native func GetSettings() -> String;
```

### CET Console

Send this line to the CET, and GPT will generate an answer for you:
```
LogChannel("DEBUG", ChatCompletionRequest({{"System", "You are very helpful assistant."}, {"User", "How are you?"}}));
```
But it will execute with visible lag. To avoid it, try this line:
```
ScheduleChatCompletionRequest({{"System", "You are very helpful assistant."}, {"User", "How are you?"}});
```
Then you can collect an answer when it is done:
```
LogChannel("DEBUG", GetLastAnswerContent());
```

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
