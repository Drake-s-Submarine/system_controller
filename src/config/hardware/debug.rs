use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DebugConfig {
    gpio: DebugGpioConfig,
}

#[derive(Debug, Deserialize)]
pub struct DebugGpioConfig {
    debug_led_pin: u8,
}
