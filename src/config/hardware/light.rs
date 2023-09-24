use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LightConfig {
    pub gpio: LightGpioConfig,
}

#[derive(Debug, Deserialize)]
pub struct LightGpioConfig {
    pub light_pin: u8,
}
