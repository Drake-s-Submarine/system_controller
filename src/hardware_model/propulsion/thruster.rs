use rppal::pwm::{ Pwm, Channel, Polarity };
use crate::error::PeripheralInitError;
use crate::traits::{ Tick, SubmarineComponent };

const STEP_UP_LIMIT: f64 = 0.05;
const STEP_DOWN_LIMIT: f64 = 0.25;

pub struct Thruster {
    // TODO: Probably use pwm so thrust can be varied
    control_pin: Pwm,
    target_duty_cycle: f64,
    enabled: bool,
}

impl Thruster {
    pub fn new(channel: Channel) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            control_pin: Pwm::with_frequency(channel, 50000.0, 0.25, Polarity::Normal, true)
                .map_err(|e| PeripheralInitError{
                    message: format!("Failed to get thruster pin as pwm: {}", e)
                })?,
            target_duty_cycle: 0.0,
            enabled: true,
        })
    }

    pub fn set_target_duty_cycle(&mut self, duty_cycle: f32) {
        let mut duty_cycle = f64::from(duty_cycle);
        if duty_cycle > 1.0 {
            duty_cycle = 1.0;
        } else if duty_cycle < 0.0 {
            duty_cycle = 0.0;
        }

        self.target_duty_cycle = duty_cycle;
    }

    pub fn set_duty_cycle(&mut self, duty_cycle: f32) {
        let mut duty_cycle = f64::from(duty_cycle);
        if duty_cycle > 1.0 {
            duty_cycle = 1.0;
        } else if duty_cycle < 0.0 {
            duty_cycle = 0.0;
        }

        self.control_pin.set_duty_cycle(duty_cycle).unwrap();
    }
}

impl SubmarineComponent for Thruster {
    fn enable(&mut self) {
        self.enabled = true;
    }
    fn disable(&mut self) {
        self.enabled = false;
    }
}

impl Tick for Thruster {
    fn tick(&mut self, _tick_count: u128) {
        if !self.enabled {
            return;
        }

        let current_duty_cycle = self.control_pin.duty_cycle().unwrap();
        let delta = self.target_duty_cycle - current_duty_cycle;
        let new_duty_cycle = if delta > 0.0 + f64::EPSILON {
            let dc = current_duty_cycle + STEP_UP_LIMIT;
            if dc > self.target_duty_cycle {
                self.target_duty_cycle
            } else {
                dc
            }
        } else if delta < 0.0 - f64::EPSILON {
            let dc = current_duty_cycle - STEP_DOWN_LIMIT;
            if dc < self.target_duty_cycle {
                self.target_duty_cycle
            } else {
                dc
            }
        } else {
            self.target_duty_cycle
        };

        self.control_pin.set_duty_cycle(new_duty_cycle).unwrap();
    }
}
