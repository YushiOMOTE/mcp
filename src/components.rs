use derive_new::new;
use specs::prelude::*;
use specs_derive::Component;

#[derive(new, Default, Component, Debug, Clone)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(new, Default, Component, Debug, Clone)]
pub struct Bound {
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
pub struct Item {
    pub id: u64,
}

#[derive(new, Default, Component, Debug, Clone)]
pub struct Player {
    pub life: u64,
    pub level: u64,
}

#[derive(new, Default, Component, Debug, Clone)]
pub struct Enemy {
    pub life: u64,
}

#[derive(Default, Component, Debug, Clone)]
pub struct Animation {
    frames: Vec<Frame>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetId(pub String);

impl AssetId {
    pub fn new<T: ToString>(aid: T) -> Self {
        Self(aid.to_string())
    }
}

#[derive(new, Default, Debug, Clone)]
pub struct Frame {
    pub aid: AssetId,
    pub time: u64,
}

impl Animation {
    pub fn empty() -> Self {
        Self { frames: vec![] }
    }

    pub fn new(aid: AssetId, time: u64) -> Self {
        Self {
            frames: vec![Frame::new(aid, time)],
        }
    }

    pub fn add(mut self, aid: AssetId, time: u64) -> Self {
        self.frames.push(Frame::new(aid, time));
        self
    }

    fn sums(&self) -> Vec<(u64, u64, &AssetId)> {
        self.frames
            .iter()
            .scan(0, |state, f| {
                let min = *state;
                *state = *state + f.time;
                let max = *state;
                Some((min, max, &f.aid))
            })
            .collect()
    }

    pub fn get(&self, count: u64) -> AssetId {
        let sums = self.sums();
        let count = count % sums.last().map(|(_, max, _)| *max).unwrap_or(1);

        sums.iter()
            .find(|(min, max, _)| min <= &count && &count < max)
            .map(|(_, _, aid)| (*aid).clone())
            .expect("Invalid image state")
    }
}

#[derive(new, Component, Debug, Clone)]
pub enum Lifetime {
    Frameout,
    Timer(u64),
    Scroll(f32),
}

#[derive(new, Component, Debug, Clone)]
pub struct MustLive;
