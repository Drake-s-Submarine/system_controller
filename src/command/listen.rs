use tokio::net::{ UnixStream, UnixListener};
use tokio::io::AsyncReadExt;
use super::{
    Command,
    CommandDispatchWrapper,
    COMMAND_QUEUE,
    Module,
    serde::{
        MODULE_IDS,
        COMMAND_BUFFER_SIZE,
        validate_command_structure,
        Serde,
    },
    commands::*,
};
use std::mem::ManuallyDrop;

pub async fn listen() {
    println!("Listening for commands.");
    let command_socket: UnixListener = 
        UnixListener::bind("/tmp/sub_cmd_socket")
        .expect("Failed to get command socket.");

    loop {
        if let Ok((mut s, _)) = command_socket.accept().await {
            ingest_command(&mut s).await;
        }
    }
}

async fn ingest_command(s: &mut UnixStream) {
    let mut buf = [0; COMMAND_BUFFER_SIZE];

    let len = s.read(&mut buf).await
        .unwrap();

    if len == 0 { println!("Empty socket"); return; }

    if !validate_command_structure(&buf) {
        eprintln!("Invalid command structure:\n{:?}", buf);
        return;
    }

    let dispatchable_command = match MODULE_IDS.get(&buf[1]) {
        Some(m) => {
            let payload: &[u8] = &buf[2..COMMAND_BUFFER_SIZE-1];
            match m {

                Module::Ballast => {
                    match BallastCommand::deserialize(payload) {
                        Ok(c) => {
                            CommandDispatchWrapper {
                                module: Module::Ballast,
                                command: Command{ballast: ManuallyDrop::new(c)}
                            }
                        },
                        Err(_) => return
                    }
                },
                Module::Light => {
                    match LightCommand::deserialize(payload) {
                        Ok(c) => {
                            CommandDispatchWrapper {
                                module: Module::Light,
                                command: Command{light: ManuallyDrop::new(c)}
                            }

                        },
                        Err(_) => return
                    }
                },
                Module::Propulsion => {
                    match PropulsionCommand::deserialize(payload) {
                        Ok(c) => {
                            CommandDispatchWrapper {
                                module: Module::Propulsion,
                                command: Command{propulsion: ManuallyDrop::new(c)}
                            }
                        },
                        Err(_) => return
                    }
                },

            } // match module
        },
        None => return,
    };

    COMMAND_QUEUE.lock().unwrap().push_back(dispatchable_command);
}
