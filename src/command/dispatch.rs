use super::{ COMMAND_QUEUE, Component };
use crate::traits::SubmarineModule;

pub fn dispatch_next_command(sub: &mut crate::Submarine) {
    let command = COMMAND_QUEUE.lock().unwrap().pop_front();

    match command {
        Some(c) => {
            match c.component {
                Component::Motor => {
                    sub.ballast.handle_command(c);
                },
            }
        },
        None => {}
    }
}
