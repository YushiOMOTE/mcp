#[macro_use]
mod extra;

mod animations;
mod assets;
mod background;
mod components;
mod effect;
mod entities;
mod resources;
mod scenarios;
mod state;
mod systems;

use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings},
};

use crate::state::Play;

fn main() {
    run::<Play>(
        "Space Chintama",
        Vector::new(resources::WIDTH, resources::HEIGHT),
        Settings::default(),
    );
}
