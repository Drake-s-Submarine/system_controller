pub trait SubmarineModule {
    fn handle_command(&mut self, cmd: crate::command::Command);
}

pub trait SubmarineComponent {
    fn enable(&mut self);
    fn disable(&mut self);
}

pub trait Tick {
    fn tick(&mut self, tick_count: u128);
}
