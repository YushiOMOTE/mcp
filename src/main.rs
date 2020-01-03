mod animations;
mod assets;
mod background;
mod components;
mod effect;
mod enemies;
mod features;
mod items;
mod resources;
mod state;
mod systems;
mod user;
mod utils;

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
