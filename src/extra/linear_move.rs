use crate::components::*;
use serde::Deserialize;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Deserialize, Component)]
pub struct Tag {
    vel: (f32, f32),
    #[serde(default = "acc_default")]
    acc: (f32, f32),
}

fn acc_default() -> (f32, f32) {
    (0.0, 0.0)
}

pub struct Action;

impl<'a> System<'a> for Action {
    type SystemData = (WriteStorage<'a, Vel>, WriteStorage<'a, Tag>);

    fn run(&mut self, (mut vel, mut tag): Self::SystemData) {
        for (mut vel, mut tag) in (&mut vel, &mut tag).join() {
            vel.x = tag.vel.0;
            vel.y = tag.vel.1;
            tag.vel.0 += tag.acc.0;
            tag.vel.1 += tag.acc.1;
        }
    }
}
