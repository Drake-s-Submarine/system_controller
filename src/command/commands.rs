use super::serde;

#[derive(Clone, Copy, Debug)]
pub enum BallastCommand {
    // 1: intake mode, 0: discharge
    Activate(bool),
    Deactivate,
}

// One byte: 0: disable, 1: intake mode, 2: discharge mode
impl serde::Serde for BallastCommand {
    fn deserialize(
        command_payload: &[u8]
    ) -> Result<Box<Self>, ()> {
        match command_payload[0] {
            0 => Ok(Box::new(BallastCommand::Deactivate)),
            1 => Ok(Box::new(BallastCommand::Activate(true))),
            2 => Ok(Box::new(BallastCommand::Activate(false))),
            _ => return Err(())
        }
    }
}

//pub en
