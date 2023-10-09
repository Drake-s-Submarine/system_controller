use std::time::Duration;
use std::fmt::Display;
use super::TELEMETRY_PACKET_SIZE;

#[derive(Debug)]
pub struct SystemTelemetry {
    tick_delta: Duration,
    tick_idle_time: Duration,
    total_tick_time: Duration,
}

impl SystemTelemetry {
    pub fn new() -> Self {
        Self {
            tick_delta: Duration::ZERO,
            tick_idle_time: Duration::ZERO,
            total_tick_time: Duration::ZERO,
        }
    }

    pub fn ingest_tick(&mut self, delta: Duration, delay: Duration) {
        self.tick_delta = delta;
        self.tick_idle_time = delay;
        self.total_tick_time = delay.saturating_add(delta);
    } 

    pub fn serialize(&self) -> [u8; TELEMETRY_PACKET_SIZE] {
        let mut buffer: [u8; TELEMETRY_PACKET_SIZE] = [0; TELEMETRY_PACKET_SIZE];

        let delta = (self.tick_delta.as_micros() as u32).to_le_bytes();
        let idle = (self.tick_idle_time.as_micros() as u32).to_le_bytes();
        let total = (self.total_tick_time.as_micros() as u32).to_le_bytes();

        buffer[0] = delta[0];
        buffer[1] = delta[1];
        buffer[2] = delta[2];
        buffer[3] = delta[3];

        buffer[4] = idle[0];
        buffer[5] = idle[1];
        buffer[6] = idle[2];
        buffer[7] = idle[3];

        buffer[8] = total[0];
        buffer[9] = total[1];
        buffer[10] = total[2];
        buffer[11] = total[3];

        buffer
    }
}

impl Display for SystemTelemetry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "System:\n\t-Run: {:#?} ({:#?}%)\n\t-Idle: {:#?} ({:#?}%)",
            self.tick_delta,
            self.tick_delta.as_millis() as f64
                / self.total_tick_time.as_millis() as f64,
            self.tick_idle_time,
            self.tick_idle_time.as_millis() as f64 
                / self.total_tick_time.as_millis() as f64,
        )
    }
}
