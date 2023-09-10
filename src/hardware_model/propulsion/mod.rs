mod thruster;

use thruster::Thruster;
use crate::{
    traits::Tick,
    error::PeripheralInitError,
    pin_map::{
        AFT_THRUSTER_PIN,
        PORT_THRUSTER_PIN,
        STARBOARD_THRUSTER_PIN,
    },
    command::commands::{ PropulsionCommand, ThrustVector }
};

pub struct Propulsion {
    aft_thruster: Thruster,
    port_thruster: Thruster,
    starboard_thruster: Thruster,
    _vector: ThrustVector,
}

impl Propulsion {
    pub fn new() -> Result<Self, PeripheralInitError> {
        Ok(Self {
            aft_thruster: Thruster::new(AFT_THRUSTER_PIN)?,
            port_thruster: Thruster::new(PORT_THRUSTER_PIN)?,
            starboard_thruster: Thruster::new(STARBOARD_THRUSTER_PIN)?,
            _vector: ThrustVector{x: 0.0, y: 0.0},
        })
    }

    pub fn handle_command(&mut self, cmd: &PropulsionCommand) {
        println!("{:?}", cmd);

        // check vec for sensical values
        // conver magnitude and direction into pwm for each thruster



    }

    // Use positive Y for forward, positive X for right
    //pub fn apply_thrust(self, x:f32, y:f32) {
    //    // get vec magnitude
    //    let mag = ((x*x) + (y*y)).sqrt();

    //    // normalize
    //    let x_norm = x / mag;
    //    let y_norm = y / mag;

    //    if x_norm > 0.0 {
    //        // self.starboard_thruster.disable();
    //        // self.port_thruster.enable();
    //        // self.port_thruster.set_thrust_magnitude(x_norm);
    //    } else {
    //        // self.port_thruster.disable();
    //        // self.starboard_thruster.enable();
    //        // self.starboard_thruster.set_thrust_magnitude(x_norm);
    //    }

    //    if y_norm > 0.0 {
    //        // self.bow_thruster.disable();
    //        //self.aft_thruster.enable();
    //        //self.aft_thruster.set_thrust_magnitude(y_norm);
    //    } else {
    //        // bow thruster
    //        // (might not have)
    //    }
    //}

    //pub fn thrust(vec: ThrustVector) {
        //// normalize vector probably
        //// compute desired thrust for each thruster
        //// apply correct voltage/pwm
        
    //}
}

impl Tick for Propulsion {
    fn tick(&mut self, tick_count: u128) {
        self.aft_thruster.tick(tick_count);
        self.port_thruster.tick(tick_count);
        self.starboard_thruster.tick(tick_count);
    }
}
