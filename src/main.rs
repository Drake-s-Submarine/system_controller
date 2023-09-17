mod command;
mod definitions;
mod error;
mod hardware_model;
mod pin_map;
mod traits;

use traits::Tick;
use hardware_model::Submarine;
use std::time::Duration;

const TICK_RATE: Duration = Duration::from_millis(100);

#[tokio::main]
async fn main() {
    let mut sub = init_system().unwrap();
    run_system(&mut sub).await;
    stop_system();
}


fn init_system() -> Result<Submarine, error::PeripheralInitError> {
    println!("Initializing system..");

    command::start_command_listener();

    hardware_model::Submarine::new()
}

async fn run_system(sub: &mut Submarine) {
    println!("Starting system");
    let mut tick_count: u128 = 0;
    loop {
        let tick_start = std::time::Instant::now();

        command::dispatch(sub);

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
