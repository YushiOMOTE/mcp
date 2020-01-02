use crate::{components::*, enemies::*};
use serde::Deserialize;
use specs_derive::Component;

#[derive(Debug, Deserialize)]
struct Config {
    width: f32,
    freq: u64,
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
    type SystemData = (
        WriteStorage<'a, Pos>,
        Read<'a, Context>,
        ReadStorage<'a, Label>,
    );

    fn run(&mut self, (mut pos, ctx, label): Self::SystemData) {
        let count = ctx.count;

        for (mut pos, label) in (&mut pos, &label).join() {
            let r = 6.28 * ((count % label.cfg.freq) as f32) / (label.cfg.freq as f32);
            pos.x += r.cos() * label.cfg.width;
        }
    }
}
