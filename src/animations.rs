use crate::components::*;
use derive_new::new;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(new, Debug, Default, Clone, Deserialize)]
pub struct FrameConfig {
    pub aid: String,
    pub time: u64,
}

#[derive(new, Debug, Default, Clone, Deserialize)]
pub struct AnimationResource(HashMap<String, Vec<FrameConfig>>);

impl AnimationResource {
    pub fn get(&self, name: &str) -> Animation {
        let frames = self
            .0
            .get(name)
            .expect(&format!("No such animation name: {}", name));
        frames.iter().fold(Animation::empty(), |a, f| {
            a.add(AssetId::new(f.aid.clone()), f.time)
        })
    }
}

impl AnimationResource {
    pub fn from_static_file() -> Self {
        serde_yaml::from_str(include_str!("config/animations.yml"))
            .expect("Couldn't parse animations file")
    }
}
