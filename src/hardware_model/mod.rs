mod ballast;
mod light;
mod propulsion;

use crate::traits::Tick;
use ballast::Ballast;
use light::Light;
use propulsion::Propulsion;

pub struct Submarine {
    pub ballast: Ballast,
    pub light: Light,
    pub propulsion: Propulsion,
}

impl Submarine {
    pub fn new() -> Result<Submarine, crate::error::PeripheralInitError> {
        Ok(Submarine {
            ballast: Ballast::new()?,
            light: Light::new()?,
            propulsion: Propulsion::new()?,
        })
    }
}

impl Tick for Submarine {
    fn tick(&mut self, tick_count: u128) {
        self.ballast.tick(tick_count);
        self.light.tick(tick_count);
        self.propulsion.tick(tick_count);
    }
}
