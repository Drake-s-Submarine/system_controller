mod ballast;
mod propulsion;

use crate::traits::Tick;
use ballast::Ballast;

pub struct Submarine {
    pub ballast: Ballast,
}

impl Submarine {
    pub fn new() -> Result<Submarine, crate::error::PeripheralInitError> {
        Ok(Submarine {
            ballast: Ballast::new()?,
        })
    }
}

impl Tick for Submarine {
    fn tick(&mut self, tick_count: u128) {

    }
}
