use rppal::gpio::{ Gpio, OutputPin };
use crate::{
    traits::Tick,
    error::PeripheralInitError,
};

pub struct ThrusterController {
    control_pin: OutputPin,
    target_state: bool,
}

impl ThrusterController {
    pub fn new(control_pin: u8) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            control_pin: Gpio::new().map_err(|e| {
                PeripheralInitError{
                    message: format!(
                        "Failed to init Gpio for pin {}: {}",
                        control_pin,
                        e.to_string()
                    )
                }
            })?.get(control_pin).map_err(|e| {
                PeripheralInitError {
                    message: format!(
                        "Failed to get gpio pin {}: {}",
                        control_pin,
                        e.to_string()
                    )
                }
            })?.into_output(),

            target_state: false
        })
    }

    pub fn enable(&mut self, en: bool) {
        self.target_state = en;
    }

    pub fn get_state(&self) -> bool {
        self.control_pin.is_set_high()
    }

    fn update(&mut self) {
        if self.target_state {
            self.control_pin.set_high();
        } else {
            self.control_pin.set_low();
        }
    }
}

impl Tick for ThrusterController {
    fn tick(&mut self, _tick_count: u32) {
        self.update()
    }
}
