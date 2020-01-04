use crate::{animations::*, components::*, extra::ComponentConfig, state::Components};
use derive_new::new;
use serde::Deserialize;
use specs::prelude::*;
use std::collections::HashMap;

pub fn spawn(world: &mut World, name: &str, x: f32, y: f32) {
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
        .fold(builder, |b, m| Components.setup(b, m.clone()));

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
