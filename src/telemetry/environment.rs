use crate::hardware_model::Submarine;
use super::TELEMETRY_PACKET_SIZE;

const SERIALIZED_BUFFER_SIZE: u8 = 3;

pub struct EnvironmentTelemetry {
    pub internal_temperature_c: u8,
    pub internal_humidity_percent: u8,
    pub is_stale: bool,
}

impl EnvironmentTelemetry {
    pub fn new() -> Self {
        Self {
            internal_temperature_c: 0,
            internal_humidity_percent: 0,
            is_stale: true,
        }
    }
}

impl super::Telemeter for EnvironmentTelemetry {
    fn collect(&mut self, sub: &Submarine) {
        self.internal_temperature_c = sub.dht11.get_temperature();
        self.internal_humidity_percent = sub.dht11.get_humidity();
        self.is_stale = sub.dht11.is_last_read_valid();
    }
    fn serialize(&self) -> ([u8; TELEMETRY_PACKET_SIZE], u8) {
        let mut buffer: [u8; TELEMETRY_PACKET_SIZE] = [0; TELEMETRY_PACKET_SIZE];

        buffer[0] = self.internal_temperature_c;
        buffer[1] = self.internal_humidity_percent;
        buffer[2] = self.is_stale as u8;

        (buffer, SERIALIZED_BUFFER_SIZE)
    }
}
