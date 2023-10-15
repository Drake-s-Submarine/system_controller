use super::Thruster;
use rppal::{ gpio::{ Gpio, OutputPin }, pwm::Channel };
use crate::{
    traits::{ Tick, SubmarineComponent },
    error::PeripheralInitError,
    config::hardware::propulsion::PropulsionConfig,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum ActiveThruster {
    Port,
    Starboard,
    None,
}

pub struct YawThrust {
    yaw_switch: OutputPin,
    port_thruster: Thruster,
    starboard_thruster: Thruster,
    active_thruster: ActiveThruster,
}

impl YawThrust {
    pub fn new(pwm_channel: Channel, config: &PropulsionConfig) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            yaw_switch: Gpio::new().map_err(|e| {
                PeripheralInitError{
                    message: format!(
                        "Failed to init Gpio for pin {}: {}",
                        config.gpio.yaw_switch_pin,
                        e.to_string()
                    )
                }
            })?.get(config.gpio.yaw_switch_pin).map_err(|e| {
                PeripheralInitError {
                    message: format!(
                        "Failed to get gpio pin {}: {}",
                        config.gpio.yaw_switch_pin,
                        e.to_string()
                    )
                }
            })?.into_output(),
            port_thruster: Thruster::new(pwm_channel, config)?,
            starboard_thruster: Thruster::new(pwm_channel, config)?,
            active_thruster: ActiveThruster::None,
        })
    }

    pub fn set_port_thrust(&mut self, magnitude: f32) {
        self.yaw_switch.set_high();
        self.port_thruster.enable();
        self.starboard_thruster.disable();

        self.port_thruster.set_target_duty_cycle(magnitude);

        if self.active_thruster != ActiveThruster::Port {
            self.port_thruster.set_duty_cycle(0.0);
            self.active_thruster = ActiveThruster::Port;
        }
    }

    pub fn set_starboard_thrust(&mut self, magnitude: f32) {
        self.yaw_switch.set_low();
        self.port_thruster.disable();
        self.starboard_thruster.enable();

        self.starboard_thruster.set_target_duty_cycle(magnitude);

        if self.active_thruster != ActiveThruster::Starboard {
            self.starboard_thruster.set_duty_cycle(0.0);
            self.active_thruster = ActiveThruster::Starboard;
        }
    }

    pub fn get_active_thruster(&self) -> u8 {
        self.active_thruster.clone() as u8
    }

    pub fn get_current_duty_cycle(&self) -> f32 {
        self.port_thruster.get_current_duty_cycle()
    }

    pub fn get_target_duty_cycle(&self) -> f32 {
        self.port_thruster.get_target_duty_cycle()
    }

    #[allow(dead_code)]
    pub fn stop(&mut self) {
        self.port_thruster.disable();
        self.starboard_thruster.disable();
        self.active_thruster = ActiveThruster::None;
    }
}

impl Tick for YawThrust {
    fn tick(&mut self, tick_count: u32) {
        self.port_thruster.tick(tick_count);
        self.starboard_thruster.tick(tick_count);
    }
}
