use machine::{InfiniteSink, InfiniteSource};
use power::{PowerNet, PowerRating, PowerStrip};

mod power;
mod machine;

fn main() {
    let mut net = PowerNet::new();
    let source = InfiniteSource::new(8192, 16);
    let sink = InfiniteSink::new(8192, 4);
    let strip = net.add_strip(PowerRating::from((8192, 16)));
    strip.borrow_mut().connect(source);
    strip.borrow_mut().connect(sink);

    dbg!(strip.borrow().net_power());
}

#[derive(Clone, Copy, Debug)]
pub struct Plot {
    pub pos: Pos,
    pub size: Size2D,
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone, Copy, Debug)]
pub struct Size2D {
    pub x: usize,
    pub y: usize,
}