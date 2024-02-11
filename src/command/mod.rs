mod dispatch;
mod listen;

use std::collections::VecDeque;
use std::sync::{ Arc, Mutex };
use once_cell::sync::Lazy;
use std::fs::remove_file;
use crate::config::CommandingConfig;
use common::commands::*;


static COMMAND_QUEUE: Lazy<Arc<Mutex<VecDeque<CommandDispatchWrapper>>>> =
    Lazy::new(|| {
        Arc::new(Mutex::new(VecDeque::new()))
    });

struct CommandDispatchWrapper {
    module: Module,
    command: Command,
}

pub union Command {
    ballast: std::mem::ManuallyDrop<Arc<BallastCommand>>,
    light: std::mem::ManuallyDrop<Arc<LightCommand>>,
    propulsion: std::mem::ManuallyDrop<Arc<PropulsionCommand>>,
}

pub fn start_command_listener(config: &CommandingConfig) {
    let socket = config.socket.clone();
    let _ = remove_file(socket.as_str());

    tokio::spawn(async move {
        println!("Spawning command listener thread.");
        listen::listen(socket.as_str()).await;
    });
}

pub fn dispatch(sub: &mut crate::Submarine) {
    dispatch::dispatch_next_command(sub);
}
