use super::{ COMMAND_QUEUE, Component };

pub fn dispatch_next_command() {
    let command = COMMAND_QUEUE.lock().unwrap().pop_front();

    match command {
        Some(c) => {
            match c.component {
                Component::Motor => {
                    if c.en {
                        super::DEBUG_LED.lock().unwrap().set_high();
                    } else {
                        super::DEBUG_LED.lock().unwrap().set_low();
                    }
                },
            }
        },
        None => {}
    }
}
