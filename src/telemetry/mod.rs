pub mod environment;
pub mod system;

use environment::EnvironmentTelemetry;
use crate::{
    hardware_model::Submarine,
    config::telemetry::TelemetryConfig,
};
use system::SystemTelemetry;
use std::{
    fs::{ remove_file, File },
    io::Write,
    time::Duration,
    thread,
    sync::mpsc,
};
use nix::unistd;
use tempfile::tempdir;

const TELEMETRY_PACKET_SIZE: usize = 16;

pub struct Telemetry {
    hw_packet_list: Vec<(Box<dyn Telemeter>, u8, bool)>,
    system: (SystemTelemetry, u8, bool),
    transmit_handle: thread::JoinHandle<()>,
    sender: mpsc::Sender<[u8; TELEMETRY_PACKET_SIZE]>,
    enabled: bool
}

impl Telemetry {
    pub fn new(config: &TelemetryConfig) -> Self {
        let pipe_path = tempdir().unwrap().path().join(&config.socket);
        let (transmit_handle, sender) =
            Telemetry::create_transmit_thread(pipe_path);

        Self {
            hw_packet_list: vec![
                (Box::new(EnvironmentTelemetry::new()), 0x0, true)
            ],
            system: (SystemTelemetry::new(), 0xF, true),

            transmit_handle,
            sender,
            enabled: true,
        }
    }

    pub fn collect_hw_telemetry(&mut self, sub: &Submarine) {
        for (packet, _, enabled) in self.hw_packet_list.iter_mut() {
            if *enabled {
                packet.collect(sub);
            }
        }
    }

    pub fn emit_telemetry(&mut self) {
        if !self.enabled {
            return
        }

        if let Err(e) = self.emit_hw_telemetry() {
            eprintln!("Failed to share telem with transmit thread: {:#}", e);
        }
        if let Err(e) = self.emit_system_telemetry() {
            eprintln!("Failed to share telem with transmit thread: {:#}", e);
        }
    }

    pub fn collect_system_telemetry(
        &mut self,
        delta: Duration,
        idle: Duration,
    ) {
        if self.enabled {
            self.system.0.ingest_tick(delta, idle);
        }
    }

    fn emit_hw_telemetry(&mut self) ->
        Result<(), mpsc::SendError<[u8; TELEMETRY_PACKET_SIZE]>>
    {
        for (packet, id, enabled) in self.hw_packet_list.iter_mut() {
            if *enabled {
                let mut payload = packet.serialize();
                payload[payload.len() - 1] = *id;
                self.sender.send(payload)?;
            }
        }

        Ok(())
    }

    fn emit_system_telemetry(&mut self) ->
        Result<(), mpsc::SendError<[u8; TELEMETRY_PACKET_SIZE]>>
    {
        if self.system.2 {
            let mut payload = self.system.0.serialize();
            payload[payload.len() - 1] = self.system.1;

            self.sender.send(payload)?
        }

        Ok(())
    }

    fn create_pipe(pipe_path: &std::path::PathBuf) {
        let _ = remove_file(pipe_path);
        unistd::mkfifo(pipe_path, nix::sys::stat::Mode::S_IRWXU).unwrap();
    }

    fn create_mpsc() ->
        (mpsc::Sender<[u8; TELEMETRY_PACKET_SIZE]>,
         mpsc::Receiver<[u8; TELEMETRY_PACKET_SIZE]>)
    { mpsc::channel() }

    fn create_transmit_thread(pipe_path: std::path::PathBuf) -> (
        thread::JoinHandle<()>,
        mpsc::Sender<[u8; TELEMETRY_PACKET_SIZE]>
    ) {
        let (sender, receiver) = Telemetry::create_mpsc();
        Telemetry::create_pipe(&pipe_path);

        (thread::spawn(move || {
            let mut pipe_handle = File::create(pipe_path).unwrap();
            while let Ok(telemetry) = receiver.recv() {
                if let Err(e) = pipe_handle.write_all(&telemetry) {
                    eprintln!("Error emitting telemetry: {}", e);
                } else { }
            }
        }),

        sender)
    }
}

trait Telemeter {
    fn collect(&mut self, sub: &Submarine);
    fn serialize(&self) -> [u8; TELEMETRY_PACKET_SIZE];
}
