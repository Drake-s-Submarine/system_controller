use crate::hardware_model::Submarine;
use super::TELEMETRY_PACKET_SIZE;
use crate::definitions::DirectionVector;

const SERIALIZED_BUFFER_SIZE: u8 = 9;

pub struct PropulsionTelemetry {
    pub vector: DirectionVector,
    pub aft_en: bool,
    pub sb_en: bool,
    pub port_en: bool,
}

impl PropulsionTelemetry {
    pub fn new() -> Self {
        Self {
            vector: DirectionVector{x:0.0,y:0.0},
            aft_en: false,
            sb_en: false,
            port_en: false,
        }
    }
}

impl super::Telemeter for PropulsionTelemetry {
    fn collect(&mut self, sub: &Submarine) {
        let propulsion = &sub.propulsion;
        
        self.vector = propulsion.get_direction();
        self.aft_en = propulsion.get_aft_state();
        self.sb_en = propulsion.get_sb_state();
        self.port_en = propulsion.get_port_state();
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

        buffer[8] = buffer[8] << self.aft_en as u8;
        buffer[8] = buffer[8] << self.sb_en as u8;
        buffer[8] = buffer[8] << self.port_en as u8;

        SERIALIZED_BUFFER_SIZE
    }
}
