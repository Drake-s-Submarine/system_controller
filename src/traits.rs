pub trait SubmarineModule {
    fn tick(&mut self, tick_count: u128);
}
