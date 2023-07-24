pub mod propulsion;
pub mod buoyancy;

use crate::error::PeripheralInitError;
use crate::traits::SubmarineModule;

pub struct Movement {
    propulsion: propulsion::Propulsion,
    //buoyancy: buoyancy::B
}

impl Movement {
    pub fn new() -> Result<Self, PeripheralInitError> {
        Ok(Movement {
            propulsion: propulsion::Propulsion::new()?,
        })
    }
}

impl SubmarineModule for Movement {
    fn tick(&mut self, tick_count: u128) {
       self.propulsion.tick(tick_count); 
    }
}
