mod ballast;
mod propulsion;

use crate::traits::Tick;
use ballast::Ballast;
use propulsion::Propulsion;

pub struct Submarine {
    pub ballast: Ballast,
    pub propulsion: Propulsion,
}

impl Submarine {
    pub fn new() -> Result<Submarine, crate::error::PeripheralInitError> {
        Ok(Submarine {
            ballast: Ballast::new()?,
            propulsion: Propulsion::new()?,
        })
    }
}

impl Tick for Submarine {
    fn tick(&mut self, tick_count: u128) {
        self.ballast.tick(tick_count);
        self.propulsion.tick(tick_count);
    }
}
