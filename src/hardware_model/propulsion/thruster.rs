use rppal::gpio::{ Gpio, OutputPin };
use crate::error::PeripheralInitError;
use crate::traits::{ Tick, SubmarineComponent };

pub struct Thruster {
    // TODO: Probably use pwm so thrust can be varied
    en_pin: OutputPin,
}

impl Thruster {
    pub fn new(control_pin: u8) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            //en_pin: pwm::Pwm::new().map_err(|e| {
            //    PeripheralInitError{
            //        message: format!("Failed to init Gpio for pin {}: {}", control_pin, e.to_string())
            //    }
            //})?.get(control_pin).map_err(|e| {
            //    PeripheralInitError {
            //        message: format!("Failed to get gpio pin {}: {}", control_pin, e.to_string())
            //    }
            //})?.into_output(),
            en_pin: Gpio::new().map_err(|e| {
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

    //pub fn set_thrust_magnitude(self, _mag: f32) {
    //    //if mag <= 1.0 && mag >= 0.0 {
    //    //    self.thrust_magnitude = mag;
    //    //}
    //}
}

impl SubmarineComponent for Thruster {
    fn enable(&mut self) {
        self.en_pin.set_high();
    }
    fn disable(&mut self) {
        self.en_pin.set_low();
    }
}

impl Tick for Thruster {
    fn tick(&mut self, _tick_count: u128) {
        
    }
}
