use rppal::pwm::{ Pwm, Channel, Polarity };
use crate::error::PeripheralInitError;
use crate::traits::{ Tick, SubmarineComponent };
use crate::config::hardware::propulsion::PropulsionConfig;

pub struct Thruster {
    // TODO: Probably use pwm so thrust can be varied
    control_pin: Pwm,
    target_duty_cycle: f64,
    enabled: bool,
    step_down_limit: f64,
    step_up_limit: f64,
}

impl Thruster {
    pub fn new(channel: Channel, conf: &PropulsionConfig) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            control_pin: Pwm::with_frequency(channel, 50000.0, 0.25, Polarity::Normal, true)
                .map_err(|e| PeripheralInitError{
                    message: format!("Failed to get thruster pin as pwm: {}", e)
                })?,
            target_duty_cycle: 0.0,
            enabled: true,
            step_down_limit: conf.thrust_step_down,
            step_up_limit: conf.thrust_step_up,
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

    pub fn get_target_duty_cycle(&self) -> f32 {
        self.target_duty_cycle as f32
    }

    pub fn get_current_duty_cycle(&self) -> f32 {
        self.control_pin.duty_cycle().unwrap() as f32
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
    fn tick(&mut self, _tick_count: u32) {
        if !self.enabled {
            return;
        }

        let current_duty_cycle = self.control_pin.duty_cycle().unwrap();
        let delta = self.target_duty_cycle - current_duty_cycle;
        let new_duty_cycle = if delta > 0.0 + f64::EPSILON {
            let dc = current_duty_cycle + self.step_up_limit;
            if dc > self.target_duty_cycle {
                self.target_duty_cycle
            } else {
                dc
            }
        } else if delta < 0.0 - f64::EPSILON {
            let dc = current_duty_cycle - self.step_down_limit;
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
