pub mod ballast;
pub mod environment;
pub mod system;

use ballast::BallastTelemetry;
use environment::EnvironmentTelemetry;
use system::SystemTelemetry;
use crate::{
    hardware_model::Submarine,
    config::telemetry::TelemetryConfig,
};
use std::{
    fs::{ remove_file, File },
    io::Write,
    time::Duration,
    thread,
    sync::mpsc,
};
use nix::unistd;
use tempfile::tempdir;

const TELEMETRY_PACKET_SIZE: usize = 32;
const ID_BYTE_OFFSET: usize = 1;
const TICK_BYTE_OFFSET: usize = 1;

pub struct Telemetry {
    hw_packet_list: Vec<(Box<dyn Telemeter>, u8, bool)>,
    system: (SystemTelemetry, u8, bool),
    transmit_handle: thread::JoinHandle<()>,
    sender: mpsc::Sender<[u8; TELEMETRY_PACKET_SIZE]>,
    enabled: bool,
    pipe_location: String,
    tick_count: u32,
}

impl Telemetry {
    pub fn new(config: &TelemetryConfig) -> Self {
        let (transmit_handle, sender) =
            Telemetry::create_transmit_thread(&config.socket);

        Self {
            // add new telemetry packets here
            hw_packet_list: vec![
                (Box::new(EnvironmentTelemetry::new()), 0x0, true),
                (Box::new(BallastTelemetry::new()), 0x1, true),
            ],
            system: (SystemTelemetry::new(), 0xF, true),

            transmit_handle,
            sender,
            enabled: true,
            pipe_location: String::from(config.socket.clone()),
            tick_count: 0,
        }
    }

    pub fn set_tick_count(&mut self, tick_count: u32) {
        self.tick_count = tick_count;
    }

    pub fn collect_hw_telemetry(&mut self, sub: &Submarine) {
        if !self.enabled { return; }

        for (packet, _, enabled) in self.hw_packet_list.iter_mut() {
            if *enabled {
                packet.collect(sub);
            }
        }
    }

    pub fn collect_system_telemetry(
        &mut self,
        delta: Duration,
        idle: Duration,
    ) {
        if !self.enabled { return; }
        if self.system.2 {
            self.system.0.ingest_tick(delta, idle);
        }
    }

    pub fn emit_telemetry(&mut self) {
        if !self.enabled {
            return
        }

        if self.transmit_handle.is_finished() {
            let (transmit_handle, sender) =
                Telemetry::create_transmit_thread(&self.pipe_location);

            self.transmit_handle = transmit_handle;
            self.sender = sender;
        }

        if let Err(e) = self.emit_hw_telemetry() {
            eprintln!("Failed to share telem with transmit thread: {}", e);
        }
        if let Err(e) = self.emit_system_telemetry() {
            eprintln!("Failed to share telem with transmit thread: {}", e);
        }
    }

    fn emit_hw_telemetry(&mut self) ->
        Result<(), mpsc::SendError<[u8; TELEMETRY_PACKET_SIZE]>>
    {
        for (packet, id, enabled) in self.hw_packet_list.iter_mut() {
            if *enabled {
                let (mut payload, size) = packet.serialize();

                if let Err(_) = Telemetry::apply_tick_count(&mut payload, size, self.tick_count) {
                    eprintln!("Not enough room in {:#X} buffer for tick count.", self.system.1);
                };

                payload[payload.len() - ID_BYTE_OFFSET] = *id;

                self.sender.send(payload)?;
            }
        }

        Ok(())
    }

    fn emit_system_telemetry(&mut self) ->
        Result<(), mpsc::SendError<[u8; TELEMETRY_PACKET_SIZE]>>
    {
        if self.system.2 {
            let (mut payload, size) = self.system.0.serialize();
            if let Err(_) = Telemetry::apply_tick_count(&mut payload, size, self.tick_count) {
                eprintln!("Not enough room in {:#X} buffer for tick count.", self.system.1);
            };
            payload[payload.len() - ID_BYTE_OFFSET] = self.system.1;

            self.sender.send(payload)?;
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

    fn create_transmit_thread(pipe_location: &str) -> (
        thread::JoinHandle<()>,
        mpsc::Sender<[u8; TELEMETRY_PACKET_SIZE]>
    ) {
        let (sender, receiver) = Telemetry::create_mpsc();
        let pipe_path = tempdir().unwrap().path().join(pipe_location);

        (thread::spawn(move || {
            let mut pipe_handle = File::create(&pipe_path).unwrap();
            Telemetry::create_pipe(&pipe_path);
            while let Ok(telemetry) = receiver.recv() {
                if let Err(e) = pipe_handle.write_all(&telemetry) {
                    eprintln!("Error emitting telemetry: {}", e);
                    break;
                }
            }
        }),

        sender)
    }

    fn apply_tick_count(
        buffer: &mut [u8; TELEMETRY_PACKET_SIZE],
        sz: u8,
        tick_count: u32
    ) -> Result<(), ()> {
        let tick_count_buf = tick_count.to_le_bytes();
        let end_index = TELEMETRY_PACKET_SIZE - TICK_BYTE_OFFSET;

        if (end_index - tick_count_buf.len()) as u8 <= sz {
            return Err(());
        }

        for i in 0..tick_count_buf.len() {
            buffer[end_index - tick_count_buf.len() + i] =
                tick_count_buf[i];
        }

        Ok(())
    }
}

trait Telemeter {
    fn collect(&mut self, sub: &Submarine);
    fn serialize(&self) -> ([u8; TELEMETRY_PACKET_SIZE], u8);
}
