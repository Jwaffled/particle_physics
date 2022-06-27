// #![allow(dead_code)]
#![allow(unused)]

use simulator::Simulator;

pub mod components;
pub mod physics;
pub mod simulator;
pub mod systems;
pub mod utils;

fn main() -> Result<(), String> {
    let ctx = sdl2::init().unwrap();
    let mut sim = Simulator::new(800, 800, 60, &ctx);
    sim.start()?;
    Ok(())
}
