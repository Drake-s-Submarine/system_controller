pub mod environment;
pub mod system;

use environment::EnvironmentTelemetry;
use crate::{
    hardware_model::Submarine,
    config::telemetry::TelemetryConfig,
};
use system::SystemTelemetry;
use std::{
    fs::File,
    io::Write,
    time::Duration,
    thread,
    sync::mpsc,
};
use nix::unistd;
use tempfile::tempdir;


const TELEMETRY_PACKET_SIZE: usize = 16;

pub struct Telemetry {
    packet_list: Vec<(Box<dyn Telemeter>, u8, bool)>,
    system: (SystemTelemetry, u8, bool),
    transmit_handle: thread::JoinHandle<()>,
    sender: mpsc::Sender<[u8; TELEMETRY_PACKET_SIZE]>,
    enabled: bool
}

impl Telemetry {
    pub fn new(config: &TelemetryConfig) -> Self {
        let pipe_path = tempdir().unwrap().path().join(&config.socket);
        unistd::mkfifo(&pipe_path, nix::sys::stat::Mode::S_IRWXU).unwrap();

        let (sender, receiver): (mpsc::Sender<[u8; TELEMETRY_PACKET_SIZE]>, mpsc::Receiver<[u8; TELEMETRY_PACKET_SIZE]>) = mpsc::channel();

        let h = thread::spawn(move || {
            let mut pipe_handle = File::create(&pipe_path).unwrap();

            while let Ok(telemetry) = receiver.recv() {
                if let Err(e) = pipe_handle.write_all(&telemetry) {
                    eprintln!("Error emitting telemetry: {}", e);
                } else { }
            }
        });

        Self {
            packet_list: vec![
                (Box::new(EnvironmentTelemetry::new()), 0x0, true)
            ],

            system: (SystemTelemetry::new(), 0xF, true),
            transmit_handle: h,
            sender,
            enabled: true,
        }
    }

    pub fn collect_hw_telemetry(&mut self, sub: &Submarine) {
        for (packet, _, enabled) in self.packet_list.iter_mut() {
            if *enabled {
                packet.collect(sub);
            }
        }
    }

    pub fn emit_telemetry(&mut self) {
        if !self.enabled {
            return
        }

        for (packet, id, enabled) in self.packet_list.iter_mut() {
            if *enabled {
                let mut payload = packet.serialize();
                payload[payload.len() - 1] = *id;
                if let Err(e) = self.sender.send(payload) {
                    eprintln!( "Error emitting telemetry: {:#}", e);
                    // TODO: attempt reconnect
                }
            }
        }

        if self.system.2 {
            let mut payload = self.system.0.serialize();
            payload[payload.len() - 1] = self.system.1;

            if let Err(e) = self.sender.send(payload) {
                eprintln!( "Error emitting telemetry: {:#}", e);
                // TODO: attempt reconnect
            }
        }
    }

    pub fn collect_system_telemetry(
        &mut self,
        delta: Duration,
        idle: Duration,
    ) {
        self.system.0.ingest_tick(delta, idle);
    }
}

trait Telemeter {
    fn collect(&mut self, sub: &Submarine);
    fn serialize(&self) -> [u8; TELEMETRY_PACKET_SIZE];
}
