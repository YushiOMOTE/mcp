use crate::{components::*, resources::*, utils::*};
use derive_new::new;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_yaml::Value;
use specs::prelude::*;
use std::collections::HashMap;

mod attack;
mod motion;

pub fn parse<T: DeserializeOwned>(value: &Value) -> T {
    serde_yaml::from_value(value.clone()).expect("Couldn't parse config")
}

pub fn init(world: &mut World) {
    attack::init(world);
    motion::init(world);
}

pub fn update(world: &mut World) {
    attack::update(world);
    motion::update(world);
}

pub fn spawn_one(world: &mut World, name: &str, x: f32, y: f32) {
    let cfg = {
        let enemies = &world.fetch::<EnemiesConfig>().enemies;
        enemies
            .get(name)
            .expect(&format!("No such enemy name: {}", name))
            .clone()
    };

    let animation = cfg.animation.iter().fold(Animation::empty(), |a, f| {
        a.add(AssetId::new(f.aid), f.time)
    });

    let builder = world
        .create_entity()
        .with(Pos::new(x, y, 0.0, cfg.size.0, cfg.size.1))
        .with(Enemy::new(cfg.life))
        .with(animation)
        .with(Lifetime::Frameout)
        .with(Vel::new(0.0, 0.0));
    let builder = cfg.motion.iter().fold(builder, |b, m| motion::setup(b, m));
    let builder = cfg.attack.iter().fold(builder, |b, m| attack::setup(b, m));

    builder.build();
}

#[derive(new, Debug, Default, Clone, Serialize, Deserialize)]
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

#[derive(new, Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnemyConfig {
    life: u64,
    size: (f32, f32),
    animation: Vec<FrameConfig>,
    motion: Vec<MotionConfig>,
    attack: Vec<AttackConfig>,
}

#[derive(new, Debug, Default, Clone, Serialize, Deserialize)]
pub struct FrameConfig {
    aid: u64,
    time: u64,
}

#[derive(new, Debug, Default, Clone, Serialize, Deserialize)]
pub struct AttackConfig {
    name: String,
    damage: u64,
    frequency: u64,
    size: (f32, f32),
    animation: Vec<FrameConfig>,
    #[serde(flatten)]
    params: Value,
}

#[derive(new, Debug, Default, Clone, Serialize, Deserialize)]
pub struct MotionConfig {
    name: String,
    #[serde(flatten)]
    params: Value,
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
