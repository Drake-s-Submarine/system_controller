pub mod ballast;
pub mod light;
pub mod propulsion;
pub mod temperature;

use serde::Deserialize;
use ballast::BallastConfig;
use light::LightConfig;
use propulsion::PropulsionConfig;
use temperature::TemperatureConfig;

#[derive(Debug, Deserialize)]
pub struct HardwareConfig {
    pub ballast: BallastConfig,
    pub light: LightConfig,
    pub propulsion: PropulsionConfig,
    pub temperature: TemperatureConfig,
}
