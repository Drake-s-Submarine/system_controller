use tokio::net::{ UnixStream, UnixListener};
use tokio::io::AsyncReadExt;
use super::{ Command, Component, COMMAND_QUEUE };

pub async fn listen() {
    println!("Listening for commands.");
    let command_socket: UnixListener = 
        UnixListener::bind("/tmp/sub_cmd_socket")
        .expect("Failed to get command socket.");

    loop {
        if let Ok((mut s, _)) = command_socket.accept().await {
            ingest_commands(&mut s).await;
        }
    }
}

async fn ingest_commands(s: &mut UnixStream) {
    let mut buf = [0; 128];

    loop {
        let len = s.read(&mut buf).await
            .unwrap();

        if len == 0 { println!("Empty socket"); break; }

        let command = match String::from_utf8_lossy(&buf[..len]).as_ref() {
            "EnableMotor" => Command {component: Component::Motor, en: true},
            "DisableMotor" => Command {component: Component::Motor, en: false},
            s => {
                eprintln!("Malformed command: {}", s);
                break;
            }
        };

        println!("Got command: {:?}", command);

        COMMAND_QUEUE.lock().unwrap()
            .push_back(command);

        println!("{:?}", COMMAND_QUEUE);
    }
}
