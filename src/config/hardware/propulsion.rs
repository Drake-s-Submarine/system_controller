use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PropulsionConfig {
    pub gpio: PropulsionGpioConfig,
}

#[derive(Debug, Deserialize)]
pub struct PropulsionGpioConfig {
    pub yaw_switch_pin: u8,
}
