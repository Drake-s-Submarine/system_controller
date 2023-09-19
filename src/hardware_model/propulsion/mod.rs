mod thruster;
mod yaw_thrust;

use thruster::Thruster;
use yaw_thrust::YawThrust;
use crate::{
    traits::Tick,
    error::PeripheralInitError,
    command::commands::PropulsionCommand,
    definitions::DirectionVector,
};

pub struct Propulsion {
    yaw_thrust: YawThrust,
    aft_thruster: Thruster,
    vector: DirectionVector,
}

impl Propulsion {
    pub fn new() -> Result<Self, PeripheralInitError> {
        Ok(Self {
            yaw_thrust: YawThrust::new(rppal::pwm::Channel::Pwm1)?,
            aft_thruster: Thruster::new(rppal::pwm::Channel::Pwm0)?,
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
        self.aft_thruster.set_duty_cycle(magnitude);
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
    fn tick(&mut self, tick_count: u128) {
        self.set_forward_thrust();
        self.set_yaw_thrust();

        self.yaw_thrust.tick(tick_count);
        self.aft_thruster.tick(tick_count);
    }
}
