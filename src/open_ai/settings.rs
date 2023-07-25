use std::env;
use std::fs;
use std::path::PathBuf;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    api_base: String,
    api_key: String,
    org_id: String,
    model: String,
    max_tokens: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            api_base: "https://api.openai.com/v1".to_string(),
            api_key: String::new(),
            org_id: String::new(),
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: 512u16,
        }
    }
}

impl Settings {
    pub fn get_api_base(&self) -> &str {
        &self.api_base
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    pub fn get_org_id(&self) -> &str {
        &self.org_id
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }

    pub fn get_max_tokens(&self) -> u16 {
        self.max_tokens
    }
}

pub struct GlobalSettings {
    pub settings: Settings,
}

lazy_static! {
    pub static ref SETTINGS: GlobalSettings = {
        let exe_path: PathBuf = env::current_exe().unwrap();
        let settings_path: PathBuf = exe_path
            .ancestors()
            .nth(3)
            .unwrap()
            .join("red4ext/plugins/CyberAI/Settings.json");

        let mut settings: Settings = Settings::default();

        match fs::read_to_string(settings_path) {
            Ok(contents) => {
                settings = serde_json::from_str(&contents).unwrap();
            }
            Err(err) => {
                eprintln!("Failed to read file: {}\nDefault settings will be use", err);
            }
        }

        GlobalSettings { settings }
    };
}

pub fn get_settings() -> String {
    let settings: &Settings = &SETTINGS.settings;

    let settings_text = format!("api_base: {}\n", settings.get_api_base())
        + &format!("api_key: {}\n", settings.get_api_key())
        + &format!("org_id: {}\n", settings.get_org_id())
        + &format!("model: {}\n", settings.get_model())
        + &format!("max_tokens: {}\n", settings.get_max_tokens());
    settings_text
}
