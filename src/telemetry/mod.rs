mod ballast;
mod environment;
mod propulsion;
mod system;

use ballast::BallastTelemetry;
use environment::EnvironmentTelemetry;
use propulsion::PropulsionTelemetry;
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
    sync::{ mpsc, Arc, atomic::{ AtomicBool, Ordering } },
};
use nix::unistd;
use tempfile::tempdir;

const TELEMETRY_PACKET_SIZE: usize = 32;
const ID_BYTE_OFFSET: usize = 1;
const TICK_BYTE_OFFSET: usize = 1;

const ENVIRONMENT_PACKET_ID: u8 = 0x0;
const BALLAST_PACKET_ID: u8 = 0x1;
const PROPULSION_PACKET_ID: u8 = 0x2;
const SYSTEM_PACKET_ID: u8 = 0xF;

struct TelemetryPacket {
    payload: Box<dyn Telemeter>,
    id: u8,
    enabled: bool,
}

impl TelemetryPacket {
    pub fn new(payload: Box<dyn Telemeter>, id: u8) -> Self {
        Self {
            payload,
            id,
            enabled: true,
        }
    }
}

pub struct Telemetry {
    hw_packet_list: Vec<TelemetryPacket>,
    system: (SystemTelemetry, u8, bool),
    emit_thread_handle: thread::JoinHandle<()>,
    emit_channel: mpsc::Sender<[u8; TELEMETRY_PACKET_SIZE]>,
    pipe_location: String,
    tick_count: u32,
    enabled: bool,
    emitter_ready: Arc<AtomicBool>,
}

impl Telemetry {
    pub fn new(config: &TelemetryConfig) -> Self {
        let emitter_ready = Arc::new(AtomicBool::new(false));
        let (transmit_handle, channel) =
            Telemetry::create_transmit_thread(&config.socket, &emitter_ready);

        Self {
            // add new telemetry packets here
            hw_packet_list: vec![
                TelemetryPacket::new(Box::new(EnvironmentTelemetry::new()),
                    ENVIRONMENT_PACKET_ID),
                TelemetryPacket::new(Box::new(BallastTelemetry::new()),
                    BALLAST_PACKET_ID),
                TelemetryPacket::new(Box::new(PropulsionTelemetry::new()),
                    PROPULSION_PACKET_ID),
            ],
            system: (SystemTelemetry::new(), SYSTEM_PACKET_ID, false),

            emit_thread_handle: transmit_handle,
            emit_channel: channel,
            pipe_location: String::from(config.socket.clone()),
            tick_count: 0,
            enabled: true,
            emitter_ready,
        }
    }

    pub fn set_tick_count(&mut self, tick_count: u32) {
        self.tick_count = tick_count;
    }

    pub fn collect_hw_telemetry(&mut self, sub: &Submarine) {
        self.hw_packet_list[0].enabled = false;
        self.hw_packet_list[1].enabled = false;
        if !self.enabled
            || !self.emitter_ready.load(Ordering::SeqCst) { return; }

        for packet in self.hw_packet_list.iter_mut() {
            if packet.enabled {
                packet.payload.collect(sub);
            }
        }
    }

    pub fn collect_system_telemetry(
        &mut self,
        delta: Duration,
        idle: Duration,
    ) {
        if !self.enabled
            || !self.emitter_ready.load(Ordering::SeqCst) { return; }

        if self.system.2 {
            self.system.0.ingest_tick(delta, idle);
        }
    }

    pub fn emit_telemetry(&mut self) {
        if self.emit_thread_handle.is_finished() {
            let (transmit_handle, sender) =
                Telemetry::create_transmit_thread(&self.pipe_location, &self.emitter_ready);

            self.emit_thread_handle = transmit_handle;
            self.emit_channel = sender;
        }

        if !self.enabled || !self.emitter_ready.load(Ordering::SeqCst) {
            return
        }

        let mut buffer: [u8; TELEMETRY_PACKET_SIZE] =
            [0; TELEMETRY_PACKET_SIZE];

        if let Err(e) = self.emit_hw_telemetry(&mut buffer) {
            eprintln!("Failed to share telem with transmit thread: {}", e);
        }
        if let Err(e) = self.emit_system_telemetry(&mut buffer) {
            eprintln!("Failed to share telem with transmit thread: {}", e);
        }
    }

    fn emit_hw_telemetry(&mut self, buffer: &mut [u8; TELEMETRY_PACKET_SIZE]) ->
        Result<(), mpsc::SendError<[u8; TELEMETRY_PACKET_SIZE]>>
    {
        for packet in self.hw_packet_list.iter_mut() {
            if packet.enabled {
                buffer.fill(0);
                let size = packet.payload.serialize(buffer);

                if let Err(_) = Telemetry::apply_tick_count(buffer, size, self.tick_count) {
                    eprintln!("Not enough room in {:#X} buffer for tick count.", self.system.1);
                };
                if let Err(_) = Telemetry::apply_packet_id(buffer, size, packet.id) {
                    eprintln!("Not enough room in {:#X} buffer for packet ID.", packet.id);
                    return Ok(());
                }
                self.emit_channel.send(buffer.clone())?;
            }
        }

        Ok(())
    }

    fn emit_system_telemetry(&mut self, buffer: &mut [u8; TELEMETRY_PACKET_SIZE]) ->
        Result<(), mpsc::SendError<[u8; TELEMETRY_PACKET_SIZE]>>
    {
        if self.system.2 {
            let size = self.system.0.serialize(buffer);
            if let Err(_) = Telemetry::apply_tick_count(buffer, size, self.tick_count) {
                eprintln!("Not enough room in {:#X} buffer for tick count.", self.system.1);
            };
            if let Err(_) = Telemetry::apply_packet_id(buffer, size, self.system.1) {
                eprintln!("Not enough room in {:#X} buffer for packet ID.", self.system.1);
                return Ok(());
            }

            self.emit_channel.send(buffer.clone())?;
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

    fn create_transmit_thread(pipe_location: &str, ready_flag: &Arc<AtomicBool>) -> (
        thread::JoinHandle<()>,
        mpsc::Sender<[u8; TELEMETRY_PACKET_SIZE]>
    ) {
        let (sender, receiver) = Telemetry::create_mpsc();
        let pipe_path = tempdir().unwrap().path().join(pipe_location);
        let emitter_ready = ready_flag.clone();

        (thread::spawn(move || {
            Telemetry::create_pipe(&pipe_path);
            let mut pipe_handle = File::create(&pipe_path).unwrap();

            emitter_ready.store(true, Ordering::SeqCst);
            while let Ok(telemetry) = receiver.recv() {
                if let Err(e) = pipe_handle.write_all(&telemetry) {
                    eprintln!("Error emitting telemetry: {}", e);
                    break;
                }
            }

            emitter_ready.store(false, Ordering::SeqCst);
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

    fn apply_packet_id(
        buffer: &mut [u8; TELEMETRY_PACKET_SIZE],
        sz: u8,
        id: u8
    ) -> Result<(), ()> {
        let id_idx = buffer.len() - ID_BYTE_OFFSET;

        if id_idx as u8 <= sz {
            return Err(())
        }

        buffer[id_idx] = id;

        Ok(())
    }
}

trait Telemeter {
    fn collect(&mut self, sub: &Submarine);
    fn serialize(&self, buffer: &mut [u8; TELEMETRY_PACKET_SIZE]) -> u8;
}
