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
    fn tick(&mut self) {
        self.movement.tick();
    }
}

fn init_system() -> Result<Submarine, error::PeripheralInitError> {
    println!("Initializing system..");
    Submarine::new()
}

fn run_system(sub: &mut Submarine) {
    println!("Starting system");
    loop {
        // assert state

        sub.tick();
        std::thread::sleep(TICK_RATE);
    }
}

fn stop_system() {
    println!("Shutting down");

}
