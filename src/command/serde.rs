use std::collections::HashMap;
use once_cell::sync::Lazy;
use super::Module;
/*
 *  [(start_byte)(module_id_byte)(payload..)(TODO: checksum)(end_byte)]
 */
pub const COMMAND_BUFFER_SIZE: usize = 16;
#[allow(dead_code)]
pub const COMMAND_PAYLOAD_SIZE: usize = COMMAND_BUFFER_SIZE - 3;
const COMMAND_BUFFER_START_BYTE: u8 = 0xA;
const COMMAND_BUFFER_END_BYTE: u8 = 0xF;

pub const MODULE_IDS: Lazy<HashMap<u8, Module>> = Lazy::new(|| {
    let mut map: HashMap<u8, Module> = HashMap::new();
    const BALLAST_ID: u8 = 0x0;
    const PROP_ID: u8 = 0x1;

    map.insert(BALLAST_ID, Module::Ballast);
    map.insert(PROP_ID, Module::Propulsion);

    map
});

pub trait Serde {
    fn deserialize(
        command_payload: &[u8]
    ) -> Result<Box<Self>, ()>;
}

pub fn validate_command_structure(
    command_buffer: &[u8; COMMAND_BUFFER_SIZE]
) -> bool {
    let start: u8 = command_buffer[0];
    let end: u8 = command_buffer[COMMAND_BUFFER_SIZE - 1];
    let module_id: u8 = command_buffer[1];

    if start != COMMAND_BUFFER_START_BYTE
    || end != COMMAND_BUFFER_END_BYTE {
        eprintln!(
            "Start ({}|{}) or end ({}|{}) byte is not correct.",
            start,
            COMMAND_BUFFER_START_BYTE,
            end,
            COMMAND_BUFFER_END_BYTE
        );
        return false;
    }

    if !MODULE_IDS.contains_key(&module_id) {
        eprintln!("Invalid module ID: {}\nValid keys: {:?}",
                  module_id,
                  MODULE_IDS.keys());
        return false;
    }

    // TODO: checksum

    true
}
