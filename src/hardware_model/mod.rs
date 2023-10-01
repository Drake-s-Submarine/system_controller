mod ballast;
mod light;
mod propulsion;
mod dht11;

use crate::{ traits::Tick, config::hardware::HardwareConfig };
use ballast::Ballast;
use light::Light;
use propulsion::Propulsion;
use dht11::Dht11;

pub struct Submarine {
    pub ballast: Ballast,
    pub light: Light,
    pub propulsion: Propulsion,
    pub dht11: Dht11,
}

impl Submarine {
    pub fn new(config: &HardwareConfig)
        -> Result<Submarine, crate::error::PeripheralInitError>
    {
        Ok(Submarine {
            ballast: Ballast::new(&config.ballast)?,
            light: Light::new(&config.light)?,
            propulsion: Propulsion::new(&config.propulsion)?,
            dht11: Dht11::new(&config.dht11)?,
        })
    }
}

impl Tick for Submarine {
    fn tick(&mut self, tick_count: u128) {
        self.ballast.tick(tick_count);
        self.light.tick(tick_count);
        self.propulsion.tick(tick_count);
        self.dht11.tick(tick_count);
    }
}
