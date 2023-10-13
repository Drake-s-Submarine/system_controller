mod thruster;
mod yaw_thrust;

use thruster::Thruster;
use yaw_thrust::YawThrust;
use crate::{
    traits::Tick,
    error::PeripheralInitError,
    command::commands::PropulsionCommand,
    config::hardware::propulsion::PropulsionConfig,
    definitions::DirectionVector,
};

pub struct Propulsion {
    yaw_thrust: YawThrust,
    aft_thruster: Thruster,
    vector: DirectionVector,
}

impl Propulsion {
    pub fn new(config: &PropulsionConfig) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            yaw_thrust: YawThrust::new(rppal::pwm::Channel::Pwm1, config)?,
            aft_thruster: Thruster::new(rppal::pwm::Channel::Pwm0, config)?,
            vector: DirectionVector{x: 0.0, y: 0.0},
        })
    }

    pub fn handle_command(&mut self, cmd: &PropulsionCommand) {
        println!("{:?}", cmd);

        match cmd {
            // TODO: check vec for sensical values
            PropulsionCommand::SetThrust(v) => self.vector = v.to_owned()
        }
    }

    fn set_forward_thrust(&mut self) {
        let magnitude = self.vector.y;
        self.aft_thruster.set_target_duty_cycle(magnitude);
    }

    fn set_yaw_thrust(&mut self) {
        let magnitude = self.vector.x.abs();

        if self.vector.x > 0.0 {
            self.yaw_thrust.set_port_thrust(magnitude);
        } else {
            self.yaw_thrust.set_starboard_thrust(magnitude);
        }
    }
}

impl Tick for Propulsion {
    fn tick(&mut self, tick_count: u32) {
        self.set_forward_thrust();
        self.set_yaw_thrust();

        self.yaw_thrust.tick(tick_count);
        self.aft_thruster.tick(tick_count);
    }
}
