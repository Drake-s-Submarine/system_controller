use std::time::Duration;
use std::fmt::Display;

pub struct System {
    tick_delta: Duration,
    tick_idle_time: Duration,
    total_tick_time: Duration,
}

#[allow(dead_code)]
impl System {
    pub fn new(
        tick_count: u128,
        delay: Duration,
        delta: Duration,
    ) -> Self {
        Self {
            tick_delta: Duration::ZERO,
            tick_idle_time: Duration::ZERO,
            total_tick_time: Duration::ZERO
        }
    }

    pub fn ingest_tick(&mut self, delay: Duration, delta: Duration) {
        self.tick_delta = delta;
        self.tick_idle_time = delay;
        self.total_tick_time = delay.saturating_add(delta);
    } 
}

impl Display for System {
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
