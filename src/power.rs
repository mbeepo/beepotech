use std::{cell::RefCell, ops::{Add, AddAssign}, rc::Rc};

#[derive(Clone, Debug, Default)]
pub struct PowerNet {
    pub strips: Vec<Rc<RefCell<PowerStrip>>>,
}

impl PowerNet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_strip(&mut self, rating: PowerRating) -> Rc<RefCell<PowerStrip>> {
        let strip = Rc::new(RefCell::new(PowerStrip::new(rating)));
        self.strips.push(strip.clone());
        strip
    }
}

#[derive(Clone, Debug)]
pub struct PowerStrip {
    pub connections: Vec<PowerConnection>,
    pub rating: PowerRating,
}

impl PowerStrip {
    pub fn new(rating: PowerRating) -> Self {
        Self { connections: Vec::with_capacity(2), rating }
    }
    
    pub fn connect<T: Powered>(&mut self, device: T) {
        self.connections.push(PowerConnection::new(device));
    }

    pub fn net_power(&self) -> NetPower {
        // TODO:
        // - Errors for invalid power configurations (insufficient source voltage, over maximum current for cable)
        self.connections.iter().fold(NetPower { volts: 0, amps: 0 }, |acc, conn| acc + conn.power)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PowerConnection {
    /// The amount of power this connection consumes from or provides to the network
    pub power: Power,
}

impl PowerConnection {
    pub fn new<T: Powered>(device: T) -> Self {
        Self { power: device.power() }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PowerRating {
    pub volts: usize,
    pub amps: usize,
}

impl From<(usize, usize)> for PowerRating {
    fn from(value: (usize, usize)) -> Self {
        Self { volts: value.0, amps: value.1 }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Power {
    pub kind: PowerKind,
    pub volts: usize,
    pub amps: usize,
}

impl From<(isize, usize)> for Power {
    fn from(value: (isize, usize)) -> Self {
        if value.0.is_negative() {
            Self {
                kind: PowerKind::Sink,
                volts: value.0.abs() as usize,
                amps: value.1,
            }
        } else {
            Self {
                kind: PowerKind::Source,
                volts: value.0 as usize,
                amps: value.1,
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NetPower {
    pub volts: isize,
    pub amps: isize,
}

impl Add<Power> for NetPower {
    type Output = NetPower;

    fn add(self, rhs: Power) -> Self::Output {
        match rhs.kind {
            PowerKind::Sink => {
                NetPower {
                    volts: self.volts,
                    amps: self.amps - rhs.amps as isize,
                }
            }
            PowerKind::Source => {
                NetPower {
                    volts: rhs.volts as isize,
                    amps: self.amps + rhs.amps as isize,
                }
            }
        }
    }
}

impl AddAssign<Power> for NetPower {
    fn add_assign(&mut self, rhs: Power) {
        *self = *self + rhs;
   }
}

#[derive(Clone, Copy, Debug)]
pub enum PowerKind {
    Source,
    Sink,
}

pub trait Powered {
    fn power(&self) -> Power;
}