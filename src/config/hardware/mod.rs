pub mod ballast;
pub mod light;
pub mod propulsion;

use serde::Deserialize;
use ballast::BallastConfig;
use light::LightConfig;
use propulsion::PropulsionConfig;

#[derive(Debug, Deserialize)]
pub struct HardwareConfig {
    pub ballast: BallastConfig,
    pub light: LightConfig,
    pub propulsion: PropulsionConfig,
}
