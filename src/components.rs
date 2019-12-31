use derive_new::new;
use specs::prelude::*;
use specs_derive::Component;

#[derive(new, Default, Component, Debug, Clone)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(new, Default, Component, Debug, Clone)]
pub struct Vel {
    pub x: f32,
    pub y: f32,
}

#[derive(new, Default, Component, Debug, Clone)]
pub struct Bullet {
    pub damage: u64,
    pub player: bool,
}

impl Bullet {
    pub fn player(damage: u64) -> Self {
        Self {
            damage,
            player: true,
        }
    }

    pub fn enemy(damage: u64) -> Self {
        Self {
            damage,
            player: false,
        }
    }
}

#[derive(new, Default, Component, Debug, Clone)]
pub struct Enemy {
    pub life: u64,
}

#[derive(new, Default, Component, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetId(pub u64);

#[derive(Default, Component, Debug, Clone)]
pub struct Bomb {
    pub counter: u64,
}

impl Bomb {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}
