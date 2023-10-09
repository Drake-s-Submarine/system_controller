pub mod hardware;
pub mod telemetry;

use serde::Deserialize;
use std::fs;

const CONFIG_FILE: &'static str = "config.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub system: SystemConfig,
    pub commanding: CommandingConfig,
    pub hardware: hardware::HardwareConfig,
    pub telemetry: telemetry::TelemetryConfig,
}

#[derive(Debug, Deserialize)]
pub struct SystemConfig {
    pub tick_rate: u8,
}

#[derive(Debug, Deserialize)]
pub struct CommandingConfig {
    pub socket: String,
}

impl Config {
    pub fn load() -> Self {
        let h = fs::read_to_string(CONFIG_FILE)
            .expect(format!(
                "Failed to open config file: {}",
                CONFIG_FILE).as_str()
            );

        toml::from_str::<Config>(h.as_str()).unwrap()
    }
}
