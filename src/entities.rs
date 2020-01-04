use crate::{
    animations::*, components::*, extra::ComponentConfig, resources::*, state::ExtraComponents,
    utils::*,
};
use derive_new::new;
use serde::Deserialize;
use specs::prelude::*;
use std::collections::HashMap;

pub fn spawn_one(world: &mut World, name: &str, x: f32, y: f32) {
    let cfg = {
        let cfg = &world.fetch::<EntitiesConfig>().0;
        cfg.get(name)
            .expect(&format!("No such enemy name: {}", name))
            .clone()
    };

    let animation = {
        let animations = world.fetch::<AnimationConfig>();
        animations.get(&cfg.animation)
    };

    let builder = world
        .create_entity()
        .with(Pos::new(x, y, 0.0, cfg.size.0, cfg.size.1))
        .with(animation)
        .with(Lifetime::Frameout)
        .with(Vel::new(0.0, 0.0));
    let builder = cfg
        .components
        .iter()
        .fold(builder, |b, m| ExtraComponents.setup(b, m.clone()));

    builder.build();
}

#[derive(new, Debug, Default, Clone, Deserialize)]
pub struct EntitiesConfig(HashMap<String, EntityConfig>);

impl EntitiesConfig {
    pub fn from_static_file() -> Self {
        serde_yaml::from_str(include_str!("config/entities.yml"))
            .expect("Couldn't parse entities file")
    }
}

#[derive(new, Debug, Default, Clone, Deserialize)]
pub struct EntityConfig {
    size: (f32, f32),
    animation: String,
    components: Vec<ComponentConfig>,
}

pub fn spawn(world: &mut World) {
    let count = world.fetch::<Context>().count;

    if count % 600 == 0 {
        spawn_one(world, "item", random_x(), -50.0);
    }

    let num = match count {
        0..=1000 => 400,
        1001..=2000 => 300,
        2001..=3000 => 200,
        3001..=4000 => 100,
        4001..=5000 => 400,
        5001..=6000 => 100,
        6001..=7000 => 50,
        7001..=8000 => 400,
        8001..=9000 => 20,
        9001..=10000 => 400,
        _ => 800,
    };

    if count % num == 0 {
        match (count / 100) % 5 {
            0 | 1 => spawn_one(world, "enemy3", random_x(), 0.0),
            2 | 3 => spawn_one(world, "enemy2", random_x(), 0.0),
            4 => spawn_one(world, "enemy1", random_x(), 0.0),
            _ => unreachable!(),
        }
    }

    if count == 4000 {
        spawn_one(world, "boss1", WIDTH / 2.0 - 50.0, -200.0);
    }

    if count == 7000 {
        spawn_one(world, "boss1", WIDTH / 2.0 - 400.0, -200.0);
        spawn_one(world, "boss1", WIDTH / 2.0 + 300.0, -200.0);
    }

    if count == 9000 {
        spawn_one(world, "boss1", WIDTH / 2.0 - 400.0, -200.0);
        spawn_one(world, "boss1", WIDTH / 2.0 - 100.0, -200.0);
        spawn_one(world, "boss1", WIDTH / 2.0 + 300.0, -200.0);
    }
}
