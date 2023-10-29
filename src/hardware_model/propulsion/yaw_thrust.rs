use super::ThrusterController;
use rppal::{
    gpio::{ Gpio, OutputPin },
    pwm::{ Channel, Pwm, Polarity, }
};
use crate::{
    traits::Tick,
    error::PeripheralInitError,
    config::hardware::propulsion::PropulsionConfig,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum YawThruster {
    Port,
    Starboard,
    None,
}

pub struct YawThrusterController {
    yaw_switch_pin: OutputPin,
    pwm_pin: Pwm,
    target_duty_cycle: f64,
    active_thruster: YawThruster,
    target_thruster: YawThruster,
}

impl YawThrusterController {
    pub fn new(pwm_channel: Channel, config: &PropulsionConfig) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            yaw_switch_pin: Gpio::new().map_err(|e| {
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
            pwm_pin: Pwm::with_frequency(
                pwm_channel,
                50000.0,
                0.0,
                Polarity::Normal,
                true
            ).map_err(|e| PeripheralInitError{
                message: format!("Failed to get thruster pin as pwm: {}", e)
            })?,
            target_duty_cycle: 0.0,
            active_thruster: YawThruster::None,
            target_thruster: YawThruster::None,
        })
    }

    pub fn set_thruster(&mut self, thruster: YawThruster) {
        self.target_thruster = thruster;
    }

    fn update(&mut self) {
        let current_dc = self.pwm_pin.duty_cycle().unwrap();
        let mut target_dc: f64 = 0.0;

        if self.active_thruster != self.target_thruster {
            if current_dc < f64::EPSILON {
                self.active_thruster = self.target_thruster;
            }
        } else {
            target_dc = self.target_duty_cycle;
        }

        self.pwm_pin.set_duty_cycle(super::compute_new_duty_cycle(
            current_dc, target_dc
        )).unwrap();

        match self.active_thruster {
            YawThruster::None => self.enable(false),
            YawThruster::Port => {
                self.enable(true);
                self.yaw_switch_pin.set_high();
            },
            YawThruster::Starboard => {
                self.enable(true);
                self.yaw_switch_pin.set_low();
            }
        }
    }

    pub fn get_active_thruster(&self) -> u8 {
        self.active_thruster.clone() as u8
    }

    pub fn get_current_duty_cycle(&self) -> f64 {
        self.pwm_pin.duty_cycle().unwrap()
    }

    pub fn get_target_duty_cycle(&self) -> f64 {
        self.target_duty_cycle
    }
}

impl Tick for YawThrusterController {
    fn tick(&mut self, _tick_count: u32) {
        self.update()
    }
}

impl ThrusterController for YawThrusterController {
    fn set_duty_cycle(&mut self, duty_cycle: f64) {
        self.target_duty_cycle = duty_cycle;
    }

    fn enable(&mut self, en: bool) {
        if en {
            self.pwm_pin.enable().unwrap();
        } else {
            self.pwm_pin.disable().unwrap();
        }
    }

    fn is_enabled(&self) -> bool {
        self.pwm_pin.is_enabled().unwrap()
    }
}
