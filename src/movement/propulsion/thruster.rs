use rppal::gpio;
use crate::error::PeripheralInitError;
use crate::traits::SubmarineModule;

pub struct Thruster {
    // TODO: Probably use pwm so thrust can be varied
    en_pin: gpio::OutputPin,
}

impl Thruster {
    pub fn new(control_pin: u8) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            en_pin: gpio::Gpio::new().map_err(|e| {
                PeripheralInitError{
                    message: format!("Failed to init Gpio for pin {}: {}", control_pin, e.to_string())
                }
            })?.get(control_pin).map_err(|e| {
                PeripheralInitError {
                    message: format!("Failed to get gpio pin {}: {}", control_pin, e.to_string())
                }
            })?.into_output(),
        })
    }

    pub fn enable(mut self) {
        self.en_pin.set_high();
    }

    pub fn disable(mut self) {
        self.en_pin.set_low();
    }
}

impl SubmarineModule for Thruster {
    fn tick(&mut self, tick_count: u128) {
        // 
        if tick_count % 10 == 0 {
            self.en_pin.toggle();
        }
    }
}
