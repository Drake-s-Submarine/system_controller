use rppal::gpio::{ OutputPin, Gpio };
use crate::{
    command::commands::LightCommand,
    config::hardware::light::LightConfig,
    error::PeripheralInitError,
    traits::Tick,
};

enum State {
    On,
    Off,
    Blink,
}

pub struct Light {
    en_pin: OutputPin,
    state: State,
}

impl Light {
    pub fn new(config: &LightConfig) -> Result<Self, PeripheralInitError> {
        Ok(Self{
            en_pin: Gpio::new().map_err(|e| {
                PeripheralInitError{
                    message: format!(
                        "Failed to init Gpio for pin {}: {}",
                        config.gpio.light_pin,
                        e.to_string()
                    )
                }
            })?.get(config.gpio.light_pin).map_err(|e| {
                PeripheralInitError {
                    message: format!(
                        "Failed to get gpio pin {}: {}",
                        config.gpio.light_pin,
                        e.to_string()
                    )
                }
            })?.into_output(),

            state: State::Off,
        })
    }

    pub fn handle_command(&mut self, cmd: &LightCommand) {
        match cmd {
            LightCommand::Off => self.turn_off(),
            LightCommand::On => self.turn_on(),
            LightCommand::Blink => self.blink(),
        }
    }

    fn turn_on(&mut self) {
        self.state = State::On;
    }

    fn turn_off(&mut self) {
        self.state = State::Off;
    }

    fn blink(&mut self) {
        self.state = State::Blink;
    }
}

impl Tick for Light {
    fn tick(&mut self, tick_count: u128) {
        match self.state {
            State::Off => self.en_pin.set_low(),
            State::On => self.en_pin.set_high(),
            State::Blink => {
                if tick_count % 10 == 0 {
                    self.en_pin.toggle();
                }
            }
        }
    }
}
