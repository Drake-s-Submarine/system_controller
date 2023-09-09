use {
    crate::{
        error::PeripheralInitError,
        pin_map::BALLAST_MOTOR_PIN,
        traits::{ Tick, SubmarineComponent },
        command::commands::BallastCommand,
    },
    rppal::gpio::{ OutputPin, Gpio },
};

pub struct Ballast {
    en_pin: OutputPin,

}

impl Ballast {
    pub fn new() -> Result<Self, PeripheralInitError> {
        Ok(Self {
            en_pin: Gpio::new().map_err(|e| {
                PeripheralInitError{
                    message: format!(
                        "Failed to init Gpio for pin {}: {}",
                        BALLAST_MOTOR_PIN,
                        e.to_string()
                    )
                }
            })?.get(BALLAST_MOTOR_PIN).map_err(|e| {
                PeripheralInitError {
                    message: format!(
                        "Failed to get gpio pin {}: {}",
                        BALLAST_MOTOR_PIN,
                        e.to_string()
                    )
                }
            })?.into_output(),
        })
    }

    pub fn handle_command(
        &mut self,
        cmd: &BallastCommand
    ) {
        match cmd {
            // TODO: intake and discharge modes
            BallastCommand::Activate(_intake) => {
                self.enable();
            },
            BallastCommand::Deactivate => self.disable()
        }
    }
}

impl Tick for Ballast {
    fn tick(&mut self, _tick_count: u128) {
        
    }
}

impl SubmarineComponent for Ballast {
    fn enable(&mut self) {
        self.en_pin.set_high();
    }
    fn disable(&mut self) {
        self.en_pin.set_low();
    }
}
