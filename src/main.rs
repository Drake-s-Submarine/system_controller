mod command;
mod config;
mod definitions;
mod error;
mod hardware_model;
mod telemetry;
mod traits;

use traits::Tick;
use hardware_model::Submarine;
use telemetry::Telemetry;
use std::time::Duration;
use std::sync::{ Arc, atomic::{ AtomicBool, Ordering} };
use ctrlc;

#[tokio::main]
async fn main() {
    println!("Initializing system..");
    let (mut sub, mut telemetry, sys_config) = init_system().unwrap();
    println!("Initialization complete.");
    run_system(&mut sub, &mut telemetry, sys_config.tick_rate).await;
    stop_system();
}

fn init_system() -> Result<
    (Submarine, Telemetry, config::SystemConfig),
    error::PeripheralInitError
> {
    println!("Loading configs.");
    let config = config::Config::load();

    command::start_command_listener(&config.commanding);
    println!("Instantiating telemetry.");
    let telemetry = Telemetry::new(&config.telemetry);

    Ok((
        hardware_model::Submarine::new(&config.hardware)?,
        telemetry,
        config.system,
    ))
}

async fn run_system(
    sub: &mut Submarine,
    telem: &mut Telemetry,
    tick_rate: u8,
) {
    println!("Starting system");
    let mut tick_count: u32 = 0;
    let tick_interval: Duration =
        Duration::from_millis(1000/tick_rate as u64);
    let run_system: Arc<AtomicBool> = 
        Arc::new(AtomicBool::new(true));
    let run_system_sigint = run_system.clone();

    let mut tick_delta = Duration::ZERO;
    let mut delay = Duration::ZERO;

    ctrlc::set_handler(move || {
        run_system_sigint.store(false, Ordering::SeqCst);
    })
    .expect("Failed to set up SIGINT handler.");

    while run_system.load(Ordering::SeqCst) {
        let tick_start = std::time::Instant::now();

        telem.set_tick_count(tick_count);
        telem.collect_system_telemetry(tick_delta, delay);

        command::dispatch(sub);
        sub.tick(tick_count);

        telem.collect_hw_telemetry(sub);
        telem.emit_telemetry();

        tick_count += 1;

        tick_delta = tick_start.elapsed();
        delay = tick_interval.checked_sub(tick_delta)
            .get_or_insert(Duration::ZERO).clone();

        std::thread::sleep(delay);
    }
}

fn stop_system() {
    println!("Shutting down");
}
