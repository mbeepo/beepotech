use crate::power::{Power, Powered, PowerKind};

#[derive(Clone, Copy, Debug)]
pub struct InfiniteSource {
    pub output: Power,
}

impl InfiniteSource {
    pub fn new(volts: usize, amps: usize) -> Self {
        Self { output: Power { kind: PowerKind::Source, volts, amps } }
    }
}

impl Powered for InfiniteSource {
    fn power(&self) -> Power {
        self.output
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InfiniteSink {
    pub input: Power,
}

impl InfiniteSink {
    pub fn new(volts: usize, amps: usize) -> Self {
        Self { input: Power { kind: PowerKind::Sink, volts, amps } }
    }
}

impl Powered for InfiniteSink {
    fn power(&self) -> Power {
        self.input
    }
}