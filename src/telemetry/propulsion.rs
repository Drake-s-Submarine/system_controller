use crate::hardware_model::Submarine;
use super::TELEMETRY_PACKET_SIZE;
use crate::definitions::DirectionVector;

const SERIALIZED_BUFFER_SIZE: u8 = 13;

pub struct PropulsionTelemetry {
    pub vector: DirectionVector,
    pub aft_duty_cycle: u8,
    pub aft_target_duty_cycle: u8,
    pub yaw_duty_cycle: u8,
    pub yaw_target_duty_cycle: u8,
    pub yaw_active_thruster: u8,
}

impl PropulsionTelemetry {
    pub fn new() -> Self {
        Self {
            vector: DirectionVector{x:0.0,y:0.0},
            aft_duty_cycle: 0,
            aft_target_duty_cycle: 0,
            yaw_duty_cycle: 0,
            yaw_target_duty_cycle: 0,
            yaw_active_thruster: 0,
        }
    }
}

impl super::Telemeter for PropulsionTelemetry {
    fn collect(&mut self, sub: &Submarine) {
        let propulsion = &sub.propulsion;
        
        self.vector = propulsion.get_direction();
        self.aft_duty_cycle = ( propulsion.get_aft_duty_cycle() * 100.0) as u8;
        self.aft_target_duty_cycle = ( propulsion.get_aft_target_duty_cycle() * 100.0 ) as u8;
        self.yaw_duty_cycle = ( propulsion.get_yaw_duty_cycle() * 100.0 ) as u8;
        self.yaw_target_duty_cycle = ( propulsion.get_yaw_target_duty_cycle() * 100.0 ) as u8;
        self.yaw_active_thruster = propulsion.get_active_yaw_thruster() as u8;
    }
    fn serialize(&self, buffer: &mut [u8; TELEMETRY_PACKET_SIZE]) -> u8 {
        let x_buf = self.vector.x.to_le_bytes();
        let y_buf = self.vector.y.to_le_bytes();

        buffer[0] = x_buf[0];
        buffer[1] = x_buf[1];
        buffer[2] = x_buf[2];
        buffer[3] = x_buf[3];

        buffer[4] = y_buf[0];
        buffer[5] = y_buf[1];
        buffer[6] = y_buf[2];
        buffer[7] = y_buf[3];

        buffer[8] = self.aft_duty_cycle;
        buffer[9] = self.aft_target_duty_cycle;
        buffer[10] = self.yaw_duty_cycle;
        buffer[11] = self.yaw_target_duty_cycle;
        buffer[12] = self.yaw_active_thruster;

        SERIALIZED_BUFFER_SIZE
    }
}
