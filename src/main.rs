mod movement;
mod error;
mod traits;

use traits::SubmarineModule;
use std::time::Duration;

const TICK_RATE: Duration = Duration::from_millis(100);

fn main() {
    let mut sub = init_system().unwrap();
    run_system(&mut sub);
    stop_system();
}

struct Submarine {
    movement: movement::Movement,
}

impl Submarine {
    pub fn new() -> Result<Submarine, error::PeripheralInitError> {
        Ok(Submarine {
            movement: movement::Movement::new()?,
        })
    }
}

impl traits::SubmarineModule for Submarine {
    fn tick(&mut self, tick_count: u128) {
        self.movement.tick(tick_count);
    }
}

fn init_system() -> Result<Submarine, error::PeripheralInitError> {
    println!("Initializing system..");
    Submarine::new()
}

fn run_system(sub: &mut Submarine) {
    println!("Starting system");
    let mut tick_count: u128 = 0;
    loop {
        let tick_start = std::time::Instant::now();
        // assert state

        sub.tick(tick_count);

        let tick_end = std::time::Instant::now();
        let tick_delta = tick_end.duration_since(tick_start);
        let delay = TICK_RATE.checked_sub(tick_delta)
            .get_or_insert(Duration::ZERO).clone();
        std::thread::sleep(delay);

        tick_count += 1;
    }
}

fn stop_system() {
    println!("Shutting down");

}
