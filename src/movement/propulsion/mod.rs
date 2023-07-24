mod thruster;

use thruster::Thruster;
use crate::{ error::PeripheralInitError, traits::SubmarineModule };

const PORT_THRUSTER_PIN: u8 = 19;

pub struct Propulsion {
    //aft_thruster: Thruster,
    port_thruster: Thruster,
    //starboard_thruster: Thruster,
}

pub struct ThrustVector {
    x: f32,
    y: f32,
}

impl Propulsion {
    pub fn new() -> Result<Self, PeripheralInitError> {
        Ok(Self {
            //aft_thruster: Thruster::new(todo!("Determine thruster pins"))?,
            port_thruster: Thruster::new(PORT_THRUSTER_PIN)?,
            //starboard_thruster: Thruster::new(todo!())?,
        })
    }

    pub fn thrust(vec: ThrustVector) {
        // normalize vector probably
        // compute desired thrust for each thruster
        // apply correct voltage/pwm

    }
}

impl SubmarineModule for Propulsion {
    fn tick(&mut self, tick_count: u128) {
        //self.aft_thruster.tick();
        self.port_thruster.tick(tick_count);
        //self.starboard_thruster.tick();
    }
}

//impl ThrustVector {

//}
