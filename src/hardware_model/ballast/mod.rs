use {
    crate::{
        error::PeripheralInitError,
        pin_map::{ BALLAST_INTAKE_PIN, BALLAST_DISCHARGE_PIN },
        traits::Tick,
        command::commands::BallastCommand,
    },
    rppal::gpio::{ OutputPin, Gpio },
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum BallastState {
    Discharge,
    Intake,
    Idle,
    Transition,
}

pub struct Ballast {
    discharge_mode_pin: OutputPin,
    intake_mode_pin: OutputPin,
    target_state: BallastState,
    state: BallastState,
}

impl Ballast {
    pub fn new() -> Result<Self, PeripheralInitError> {
        Ok(Self {
            discharge_mode_pin: Gpio::new().map_err(|e| {
                PeripheralInitError{
                    message: format!(
                        "Failed to init Gpio for pin {}: {}",
                        BALLAST_DISCHARGE_PIN,
                        e.to_string()
                    )
                }
            })?.get(BALLAST_DISCHARGE_PIN).map_err(|e| {
                PeripheralInitError {
                    message: format!(
                        "Failed to get gpio pin {}: {}",
                        BALLAST_DISCHARGE_PIN,
                        e.to_string()
                    )
                }
            })?.into_output(),

            intake_mode_pin: Gpio::new().map_err(|e| {
                PeripheralInitError{
                    message: format!(
                        "Failed to init Gpio for pin {}: {}",
                        BALLAST_INTAKE_PIN,
                        e.to_string()
                    )
                }
            })?.get(BALLAST_INTAKE_PIN).map_err(|e| {
                PeripheralInitError {
                    message: format!(
                        "Failed to get gpio pin {}: {}",
                        BALLAST_INTAKE_PIN,
                        e.to_string()
                    )
                }
            })?.into_output(),

            state: BallastState::Idle,
            target_state: BallastState::Idle,
        })
    }

    pub fn handle_command(
        &mut self,
        cmd: &BallastCommand
    ) {
        match cmd {
            // TODO: intake and discharge modes
            BallastCommand::Idle => self.set_idle_state(),
            BallastCommand::Intake => self.set_intake_state(),
            BallastCommand::Discharge => self.set_discharge_state(),
        }
    }
    
    fn set_discharge_state(&mut self) {
        self.state = BallastState::Transition;
        self.target_state = BallastState::Discharge;
    }
    fn set_intake_state(&mut self) {
        self.state = BallastState::Transition;
        self.target_state = BallastState::Intake;
    }
    fn set_idle_state(&mut self) {
        self.state = BallastState::Transition;
        self.target_state = BallastState::Idle;
    }

    fn stop_all(&mut self) {
        self.discharge_mode_pin.set_low();
        self.intake_mode_pin.set_low();
    }
}

impl Tick for Ballast {
    fn tick(&mut self, _tick_count: u128) {
        if self.state == BallastState::Transition {
            self.stop_all();
        }

        match self.state {
            BallastState::Idle => {
                self.stop_all();
            },
            BallastState::Intake => {
                self.intake_mode_pin.set_high();
                self.discharge_mode_pin.set_low();
            },
            BallastState::Discharge => {
                self.intake_mode_pin.set_low();
                self.discharge_mode_pin.set_high();
            },
            BallastState::Transition => self.state = self.target_state.clone(),
        };
    }
}
