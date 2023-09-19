use super::Thruster;
use rppal::{ gpio::{ Gpio, OutputPin }, pwm::Channel };
use crate::{
    traits::{ Tick, SubmarineComponent },
    error::PeripheralInitError,
    pin_map::PROP_YAW_SWITCH_PIN,
};

pub struct YawThrust {
    yaw_switch: OutputPin,
    port_thruster: Thruster,
    starboard_thruster: Thruster,
}

impl YawThrust {
    pub fn new(pwm_channel: Channel) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            yaw_switch: Gpio::new().map_err(|e| {
                PeripheralInitError{
                    message: format!(
                        "Failed to init Gpio for pin {}: {}",
                        PROP_YAW_SWITCH_PIN,
                        e.to_string()
                    )
                }
            })?.get(PROP_YAW_SWITCH_PIN).map_err(|e| {
                PeripheralInitError {
                    message: format!(
                        "Failed to get gpio pin {}: {}",
                        PROP_YAW_SWITCH_PIN,
                        e.to_string()
                    )
                }
            })?.into_output(),
            port_thruster: Thruster::new(pwm_channel)?,
            starboard_thruster: Thruster::new(pwm_channel)?,
        })
    }

    pub fn set_port_thrust(&mut self, magnitude: f32) {
        self.yaw_switch.set_high();
        self.port_thruster.enable();
        self.starboard_thruster.disable();

        self.port_thruster.set_duty_cycle(magnitude);
    }

    pub fn set_starboard_thrust(&mut self, magnitude: f32) {
        self.yaw_switch.set_low();
        self.port_thruster.disable();
        self.starboard_thruster.enable();

        self.starboard_thruster.set_duty_cycle(magnitude);
    }
}

impl Tick for YawThrust {
    fn tick(&mut self, tick_count: u128) {
        self.port_thruster.tick(tick_count);
        self.starboard_thruster.tick(tick_count);
    }
}
