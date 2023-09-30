use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PropulsionConfig {
    pub gpio: PropulsionGpioConfig,
    pub thrust_step_up: f64,
    pub thrust_step_down: f64,
}

#[derive(Debug, Deserialize)]
pub struct PropulsionGpioConfig {
    pub yaw_switch_pin: u8,
}
