mod yaw_thrust;
mod aft_thrust;

use yaw_thrust::{
    YawThrusterController,
    YawThruster,
};
use aft_thrust::AftThrusterController;
use crate::{
    traits::Tick,
    error::PeripheralInitError,
    command::commands::PropulsionCommand,
    config::hardware::propulsion::PropulsionConfig,
    definitions::DirectionVector,
};

const PWM_STEP_UP: f64 = 0.05;
const PWM_STEP_DOWN: f64 = 0.25;

pub struct Propulsion {
    yaw_thrust: YawThrusterController,
    aft_thrust: AftThrusterController,
    vector: DirectionVector,
}

impl Propulsion {
    pub fn new(config: &PropulsionConfig) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            yaw_thrust: YawThrusterController::new(rppal::pwm::Channel::Pwm1, config)?,
            aft_thrust: AftThrusterController::new(rppal::pwm::Channel::Pwm0, config)?,
            vector: DirectionVector{x: 0.0, y: 0.0},
        })
    }

    pub fn handle_command(&mut self, cmd: &PropulsionCommand) {
        println!("{:?}", cmd);

        match cmd {
            // TODO: check vec for sensical values
            PropulsionCommand::SetThrust(v) => {
                self.vector.x = v.x.clamp(-1.0, 1.0);
                self.vector.y = v.y.clamp(0.0, 1.0);
            }
        }
    }

    pub fn get_direction(&self) -> DirectionVector {
        self.vector
    }

    pub fn get_aft_duty_cycle(&self) -> f64 {
        self.aft_thrust.get_current_duty_cycle()
    }

    pub fn get_aft_target_duty_cycle(&self) -> f64 {
        self.aft_thrust.get_target_duty_cycle()
    }

    pub fn get_yaw_duty_cycle(&self) -> f64 {
        self.yaw_thrust.get_current_duty_cycle()
    }

    pub fn get_yaw_target_duty_cycle(&self) -> f64 {
        self.yaw_thrust.get_target_duty_cycle()
    }

    pub fn get_active_yaw_thruster(&self) -> u8 {
        self.yaw_thrust.get_active_thruster()
    }

    fn set_forward_thrust(&mut self) {
        let magnitude = self.vector.y as f64;
        self.aft_thrust.set_duty_cycle(magnitude);
    }

    fn set_yaw_thrust(&mut self) {
        let mut magnitude = self.vector.x.abs() as f64;

        if magnitude < f64::EPSILON {
            magnitude = 0.0;
            self.yaw_thrust.set_thruster(YawThruster::None);
        } else if self.vector.x > 0.0 {
            self.yaw_thrust.set_thruster(YawThruster::Starboard);
        } else {
            self.yaw_thrust.set_thruster(YawThruster::Port);
        }

        self.yaw_thrust.set_duty_cycle(magnitude);
    }
}

impl Tick for Propulsion {
    fn tick(&mut self, tick_count: u32) {
        self.set_forward_thrust();
        self.set_yaw_thrust();

        self.yaw_thrust.tick(tick_count);
        self.aft_thrust.tick(tick_count);
    }
}

trait ThrusterController {
    fn set_duty_cycle(&mut self, duty_cycle: f64);
    fn enable(&mut self, en: bool);
    fn is_enabled(&self) -> bool;
}

fn compute_new_duty_cycle(current_dc: f64, target_dc: f64) -> f64 {
    let delta = target_dc - current_dc;

    if delta > 0.0 + f64::EPSILON {
        let dc = current_dc + PWM_STEP_UP;
        if dc > target_dc {
            target_dc
        } else {
            dc
        }
    } else if delta < 0.0 - f64::EPSILON {
        let dc = current_dc - PWM_STEP_DOWN;
        if dc < target_dc {
            target_dc
        } else {
            dc
        }
    } else {
        target_dc
    }
}
