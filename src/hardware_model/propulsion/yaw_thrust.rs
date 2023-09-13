use super::Thruster;
use rppal::pwm::Channel;
use crate::{
    traits::{ Tick, SubmarineComponent },
    error::PeripheralInitError,
};

pub struct YawThrust {
    port_thruster: Thruster,
    //starboard_thruster: Thruster,
}

impl YawThrust {
    pub fn new(pwm_channel: Channel) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            port_thruster: Thruster::new(pwm_channel)?,
            //starboard_thruster: Thruster::new(pwm_channel)?,
        })
    }

    pub fn set_port_thrust(&mut self, magnitude: f32) {
        self.port_thruster.enable();
        //self.starboard_thruster.disable();

        // set port thrust mag
        self.port_thruster.set_duty_cycle(magnitude);
    }

    //pub fn set_starboard_thrust(&mut self, magnitude: f32) {
    //    self.port_thruster.disable();
    //    self.starboard_thruster.enable();

    //    // set sb thrust mag
    //}
}

impl Tick for YawThrust {
    fn tick(&mut self, tick_count: u128) {
        self.port_thruster.tick(tick_count);
        //self.starboard_thruster.tick(tick_count);
    }
}
