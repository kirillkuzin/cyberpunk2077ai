use std::env;
use std::fs;
use std::path::PathBuf;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Settings {
    api_key: String,
    org_id: String,
    model: String,
    max_tokens: u16,
}

impl Into<String> for &Settings {
    fn into(self) -> String {
        serde_json::to_string(self).expect("Failed to serialize JSON")
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            api_key: String::new(),
            org_id: String::new(),
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: 512u16,
        }
    }
}

impl Settings {
    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn get_org_id(&self) -> String {
        self.org_id.clone()
    }

    pub fn get_model(&self) -> String {
        self.model.clone()
    }

    pub fn get_max_tokens(&self) -> u16 {
        self.max_tokens
    }
}


struct GlobalSettings {
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

    settings.into()
}