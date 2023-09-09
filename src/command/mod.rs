pub mod commands;
mod dispatch;
mod listen;
mod serde;

use std::collections::VecDeque;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use rppal::gpio::OutputPin;
use std::fs::remove_file;
use commands::*;


static COMMAND_QUEUE: Lazy<Mutex<VecDeque<CommandDispatchWrapper>>> =
    Lazy::new(|| {
        Mutex::new(VecDeque::new())
    });

#[allow(dead_code)]
static DEBUG_LED: Lazy<Mutex<OutputPin>> = Lazy::new(|| {
    let led = rppal::gpio::Gpio::new().unwrap()
        .get(crate::pin_map::DEBUG_LED).unwrap().into_output();
    Mutex::new(led)
});

#[derive(Debug)]
pub enum Module {
    Ballast
}

struct CommandDispatchWrapper {
    module: Module,
    command: Command,
}

pub union Command {
    ballast: std::mem::ManuallyDrop<Box<BallastCommand>>,
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

//struct ToggleButton {
//    is_on: bool,
//    was_toggled: bool,
//    pub pin: InputPin,
//}

//impl ToggleButton {
//    pub fn new(pin_num: u8) -> Result<Self, rppal::gpio::Error> {
//        let pin = Gpio::new()?.get(pin_num)?.into_input();

//        Ok(Self {
//            is_on: false,
//            was_toggled: false,
//            pin
//        })
//    }

//    pub fn check_was_toggled(&mut self) -> bool {
//        let result = self.was_toggled;
//        self.was_toggled = false;

//        result
//    }

//    fn toggle(&mut self) {
//        self.was_toggled = true;
//        self.is_on = !self.is_on;
//    }

//    pub fn get_state(&self) -> bool {
//        self.is_on
//    }

//    pub fn check_pin(&mut self) {
//        if !self.was_toggled {
//            if self.pin.is_high() {
//                self.toggle();
//            }
//        }
//    }
//}
