use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Dht11Config {
    pub gpio: Dht11GpioConfig,
    pub sample_interval: u8,
}

#[derive(Debug, Deserialize)]
pub struct Dht11GpioConfig {
    pub data_pin: u8,
}
