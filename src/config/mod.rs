use serde::Deserialize;
use std::fs;

const CONFIG_FILE: &'static str = "config.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub system: System,
}

#[derive(Debug, Deserialize)]
pub struct System {
    pub tick_rate: u8,
}

impl Config {
    pub fn load() -> Self {
        let h = fs::read_to_string(CONFIG_FILE)
            .expect(format!("Failed to open config file: {}", CONFIG_FILE).as_str());

        let config = toml::from_str::<Config>(h.as_str()).unwrap();

        Self {
            system: config.system,
        }
    }
}
