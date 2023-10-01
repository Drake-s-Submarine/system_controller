pub mod ballast;
pub mod light;
pub mod propulsion;
pub mod dht11;

use serde::Deserialize;
use ballast::BallastConfig;
use light::LightConfig;
use propulsion::PropulsionConfig;
use dht11::Dht11Config;

#[derive(Debug, Deserialize)]
pub struct HardwareConfig {
    pub ballast: BallastConfig,
    pub light: LightConfig,
    pub propulsion: PropulsionConfig,
    pub dht11: Dht11Config,
}
