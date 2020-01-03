use crate::{animations::*, components::*, features, resources::*};
use serde::Deserialize;
use specs::prelude::*;

#[derive(Default, Clone, Deserialize)]
pub struct UserConfig {
    animation: String,
}

impl UserConfig {
    pub fn from_static_file() -> Self {
        serde_yaml::from_str(include_str!("config/user.yml")).expect("Couldn't parse user file")
    }
}

pub fn spawn(world: &mut World) {
    let animation = {
        let animations = world.fetch::<AnimationResource>();
        let cfg = world.fetch::<UserConfig>();
        animations.get(&cfg.animation)
    };

    world
        .create_entity()
        .with(Pos::new(WIDTH / 2.0, HEIGHT * 0.9, 0.0, 16.0, 24.0))
        .with(Player::new(150, 0))
        .with(Vel::new(0.0, 0.0))
        .with(Bound::new(0.0, 0.0, WIDTH, HEIGHT))
        .with(features::control::Tag { vel: 3.0 })
        .with(features::shooter::Tag {})
        .with(animation)
        .with(MustLive)
        .build();
}
