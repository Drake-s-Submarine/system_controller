pub trait SubmarineComponent {
    fn enable(&mut self);
    fn disable(&mut self);
}

pub trait Tick {
    fn tick(&mut self, tick_count: u32);
}

