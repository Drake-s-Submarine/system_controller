use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TemperatureConfig {
    pub gpio: TemperatureGpioConfig,
    pub sample_interval: u8,
}

#[derive(Debug, Deserialize)]
pub struct TemperatureGpioConfig {
    pub data_pin: u8,
}
