use super::ThrusterController;
use rppal::pwm::{ Channel, Pwm, Polarity, };
use crate::{
    traits::Tick,
    error::PeripheralInitError,
    config::hardware::propulsion::PropulsionConfig,
};

pub struct AftThrusterController {
    pwm_pin: Pwm,
    target_duty_cycle: f64,
}

impl AftThrusterController {
    pub fn new(pwm_channel: Channel, _config: &PropulsionConfig) -> Result<Self, PeripheralInitError> {
        Ok(Self {
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
        })
    }

    fn update(&mut self) {
        let current_dc = self.pwm_pin.duty_cycle().unwrap();

        if current_dc < f64::EPSILON {
            self.enable(false);
        } else {
            self.enable(true);
        }

        self.pwm_pin.set_duty_cycle(super::compute_new_duty_cycle(
            current_dc, self.target_duty_cycle
        )).unwrap();
    }

    pub fn get_current_duty_cycle(&self) -> f64 {
        self.pwm_pin.duty_cycle().unwrap()
    }

    pub fn get_target_duty_cycle(&self) -> f64 {
        self.target_duty_cycle
    }
}

impl Tick for AftThrusterController {
    fn tick(&mut self, _tick_count: u32) {
        self.update()
    }
}

impl ThrusterController for AftThrusterController {
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
