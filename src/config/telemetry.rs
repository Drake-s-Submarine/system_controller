use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TelemetryConfig {
    pub socket: String
}
