use crate::{components::*, enemies::*};
use serde::Deserialize;
use specs_derive::Component;

#[derive(Debug, Deserialize)]
struct Config {
    vel: (f32, f32),
}

#[derive(Debug, Component)]
pub struct Label {
    cfg: Config,
}

impl Label {
    pub fn new(cfg: MotionConfig) -> Self {
        Self {
            cfg: parse(&cfg.params),
        }
    }
}

pub struct Action;

impl<'a> System<'a> for Action {
    type SystemData = (WriteStorage<'a, Vel>, ReadStorage<'a, Label>);

    fn run(&mut self, (mut vel, label): Self::SystemData) {
        for (mut vel, label) in (&mut vel, &label).join() {
            vel.x = label.cfg.vel.0;
            vel.y = label.cfg.vel.1;
        }
    }
}
