pub mod commands;
mod dispatch;
mod listen;
mod serde;

use std::collections::VecDeque;
use std::sync::{ Arc, Mutex };
use once_cell::sync::Lazy;
use rppal::gpio::OutputPin;
use std::fs::remove_file;
use commands::*;


static COMMAND_QUEUE: Lazy<Arc<Mutex<VecDeque<CommandDispatchWrapper>>>> =
    Lazy::new(|| {
        Arc::new(Mutex::new(VecDeque::new()))
    });

#[allow(dead_code)]
static DEBUG_LED: Lazy<Mutex<OutputPin>> = Lazy::new(|| {
    let led = rppal::gpio::Gpio::new().unwrap()
        .get(crate::pin_map::DEBUG_LED).unwrap().into_output();
    Mutex::new(led)
});

#[derive(Debug)]
pub enum Module {
    Ballast,
    Light,
    Propulsion,
}

struct CommandDispatchWrapper {
    module: Module,
    command: Command,
}

pub union Command {
    ballast: std::mem::ManuallyDrop<Arc<BallastCommand>>,
    light: std::mem::ManuallyDrop<Arc<LightCommand>>,
    propulsion: std::mem::ManuallyDrop<Arc<PropulsionCommand>>,
}

pub fn start_command_listener() {
    let _ = remove_file("/tmp/sub_cmd_socket");

    tokio::spawn(async move {
        println!("Spawning command listener thread.");
        listen::listen().await;
    });
}

pub fn dispatch(sub: &mut crate::Submarine) {
    dispatch::dispatch_next_command(sub);
}
