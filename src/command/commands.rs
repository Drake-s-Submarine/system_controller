use super::serde;

#[derive(Debug)]
pub enum BallastCommand {
    // 1: intake mode, 0: discharge
    Activate(bool),
    Deactivate,
}

// One byte: 0: disable, 1: intake mode, 2: discharge mode
// [ [] [][][][][][][][][][][] ]
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


#[derive(Debug)]
pub enum PropulsionCommand {
    SetThrust(ThrustVector),
}

// TODO: This doesn't seem like the right place for this
#[derive(Debug)]
pub struct ThrustVector {
    pub x: f32,
    pub y: f32,
}

//   x: f32   y: f32   unused
// [ [][][][] [][][][] [][][][] ]
impl serde::Serde for PropulsionCommand {
    fn deserialize(
        command_payload: &[u8]
    ) -> Result<Box<Self>, ()> {
        let mut x_component: [u8; 4] = [0; 4];
        let mut y_component: [u8; 4] = [0; 4];

        x_component[0] = command_payload[0];
        x_component[1] = command_payload[1];
        x_component[2] = command_payload[2];
        x_component[3] = command_payload[3];

        y_component[0] = command_payload[4];
        y_component[1] = command_payload[5];
        y_component[2] = command_payload[6];
        y_component[3] = command_payload[7];

        let x_component: f32 = f32::from_le_bytes(x_component);
        let y_component: f32 = f32::from_le_bytes(y_component);

        Ok(Box::new(PropulsionCommand::SetThrust(ThrustVector {
            x: x_component,
            y: y_component,
        })))
    }
}
