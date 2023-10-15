use crate::hardware_model::Submarine;
use super::TELEMETRY_PACKET_SIZE;

const SERIALIZED_BUFFER_SIZE: u8 = 2;

pub struct BallastTelemetry {
    pub current_state: u8,
    pub target_state: u8,
}

impl BallastTelemetry {
    pub fn new() -> Self {
        Self {
            current_state: 0x0,
            target_state: 0x0,
        }
    }
}

impl super::Telemeter for BallastTelemetry {
    fn collect(&mut self, sub: &Submarine) {
        let ballast = &sub.ballast;

        self.current_state = ballast.get_current_state() as u8;
        self.target_state = ballast.get_target_state() as u8;
    }
    fn serialize(&self, buffer: &mut [u8; TELEMETRY_PACKET_SIZE]) -> u8 {
        buffer[0] = self.current_state;
        buffer[1] = self.target_state;

        SERIALIZED_BUFFER_SIZE
    }
}
