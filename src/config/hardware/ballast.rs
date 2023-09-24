use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BallastConfig {
    pub gpio: BallastGpioConfig
}

#[derive(Debug, Deserialize)]
pub struct BallastGpioConfig {
    pub intake_pin: u8,
    pub discharge_pin: u8,
}
