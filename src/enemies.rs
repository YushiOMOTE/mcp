use crate::{
    animations::*,
    components::*,
    features::{self, *},
    resources::*,
    utils::*,
};
use derive_new::new;
use serde::Deserialize;
use serde_yaml::Value;
use specs::prelude::*;
use std::collections::HashMap;

pub fn spawn_one(world: &mut World, name: &str, x: f32, y: f32) {
    let cfg = {
        let enemies = &world.fetch::<EnemiesConfig>().enemies;
        enemies
            .get(name)
            .expect(&format!("No such enemy name: {}", name))
            .clone()
    };

    let animation = {
        let animations = world.fetch::<AnimationResource>();
        animations
            .get(&cfg.animation)
            .iter()
            .fold(Animation::empty(), |a, f| {
                a.add(AssetId::new(f.aid), f.time)
            })
    };

    let builder = world
        .create_entity()
        .with(Pos::new(x, y, 0.0, cfg.size.0, cfg.size.1))
        .with(Enemy::new(cfg.life))
        .with(animation)
        .with(Lifetime::Frameout)
        .with(Vel::new(0.0, 0.0));
    let builder = cfg
        .features
        .iter()
        .fold(builder, |b, m| features::setup(b, m.clone()));

    builder.build();
}

#[derive(new, Debug, Default, Clone, Deserialize)]
pub struct EnemiesConfig {
    #[serde(flatten)]
    enemies: HashMap<String, EnemyConfig>,
}

impl EnemiesConfig {
    pub fn from_static_file() -> Self {
        serde_yaml::from_str(include_str!("config/enemies.yml"))
            .expect("Couldn't parse enemies file")
    }
}

#[derive(new, Debug, Default, Clone, Deserialize)]
pub struct EnemyConfig {
    life: u64,
    size: (f32, f32),
    animation: String,
    features: Vec<FeatureConfig>,
}

pub fn spawn(world: &mut World) {
    let count = world.fetch::<Context>().count;

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
