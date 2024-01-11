mod thruster_controller;

use thruster_controller::ThrusterController;
use crate::{
    traits::Tick,
    error::PeripheralInitError,
    command::commands::PropulsionCommand,
    config::hardware::propulsion::PropulsionConfig,
    definitions::DirectionVector,
};

pub struct Propulsion {
    aft_thruster: ThrusterController,
    starboard_thruster: ThrusterController,
    port_thruster: ThrusterController,
    vector: DirectionVector,
}

impl Propulsion {
    pub fn new(config: &PropulsionConfig) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            aft_thruster: ThrusterController::new(config.gpio.aft_pin)?,
            starboard_thruster: ThrusterController::new(config.gpio.starboard_pin)?,
            port_thruster: ThrusterController::new(config.gpio.port_pin)?,
            vector: DirectionVector{x: 0.0, y: 0.0},
        })
    }

    pub fn handle_command(&mut self, cmd: &PropulsionCommand) {
        println!("{:?}", cmd);

        match cmd {
            PropulsionCommand::SetThrust(v) => {
                self.vector.x = v.x.clamp(-1.0, 1.0);
                self.vector.y = v.y.clamp(0.0, 1.0);
            }
        }
    }

    pub fn get_direction(&self) -> DirectionVector {
        self.vector
    }

    pub fn get_aft_state(&self) -> bool {
        self.aft_thruster.get_state()
    }
    pub fn get_sb_state(&self) -> bool {
        self.starboard_thruster.get_state()
    }
    pub fn get_port_state(&self) -> bool {
        self.port_thruster.get_state()
    }

    fn set_thruster_states(&mut self) {
        let aft_en = self.vector.y > 0.3;
        let sb_en = self.vector.x > 0.3;
        let port_en = self.vector.x < -0.3;

        if sb_en && port_en {
            // this shouldn't happen and I should probably do something
            // TODO: handle this
        }
        
        self.aft_thruster.enable(aft_en);
        self.starboard_thruster.enable(sb_en);
        self.port_thruster.enable(port_en);
    }
}

impl Tick for Propulsion {
    fn tick(&mut self, tick_count: u32) {
        self.set_thruster_states();

        self.aft_thruster.tick(tick_count);
        self.starboard_thruster.tick(tick_count);
        self.port_thruster.tick(tick_count);
    }
}
