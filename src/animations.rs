use derive_new::new;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(new, Debug, Default, Clone, Deserialize)]
pub struct FrameConfig {
    pub aid: u64,
    pub time: u64,
}

#[derive(new, Debug, Default, Clone, Deserialize)]
pub struct AnimationResource(HashMap<String, Vec<FrameConfig>>);

impl AnimationResource {
    pub fn get(&self, name: &str) -> &[FrameConfig] {
        self.0
            .get(name)
            .expect(&format!("No such animation name: {}", name))
    }
}

impl AnimationResource {
    pub fn from_static_file() -> Self {
        serde_yaml::from_str(include_str!("config/animations.yml"))
            .expect("Couldn't parse enemies file")
    }
}
