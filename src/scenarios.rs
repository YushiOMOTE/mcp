use crate::{entities, resources::*};
use serde::Deserialize;
use specs::prelude::*;

#[derive(Deserialize, Clone, Debug)]
pub struct ScenarioConfig(Vec<TimeConfig>);

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Time {
    Range(u64, u64),
    At(u64),
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Coord {
    Random(f32, f32),
    At(f32),
}

impl Coord {
    fn value(&self) -> f32 {
        use rand::Rng;

        match self {
            Coord::Random(min, max) => rand::thread_rng().gen_range(min, max),
            Coord::At(at) => *at,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct TimeConfig {
    time: Time,
    spawn: Vec<SpawnConfig>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SpawnConfig {
    freq: Option<u64>,
    spawn: String,
    pos: (Coord, Coord),
}

impl ScenarioConfig {
    pub fn from_static_file() -> Self {
        serde_yaml::from_str(include_str!("config/scenario.yml"))
            .expect("Couldn't parse scenario file")
    }
}

pub fn spawn(world: &mut World) {
    let cfg = (*world.fetch::<ScenarioConfig>()).clone();
    let count = world.fetch::<Context>().count;

    for cfg in cfg.0.iter() {
        match cfg.time {
            Time::Range(begin, end) => {
                if count < begin || end <= count {
                    continue;
                }
            }
            Time::At(at) => {
                if count != at {
                    continue;
                }
            }
        }

        for s in &cfg.spawn {
            if let Some(freq) = s.freq {
                if count % freq != 0 {
                    continue;
                }
            }

            let (x, y) = &s.pos;
            entities::spawn(world, &s.spawn, x.value(), y.value());
        }
    }
}
