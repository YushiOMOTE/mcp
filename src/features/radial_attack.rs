use crate::{animations::*, components::*, resources::*};
use serde::Deserialize;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Component, Deserialize)]
pub struct Tag {
    damage: u64,
    size: (f32, f32),
    frequency: u64,
    animation: String,
    vel: f32,
    #[serde(default = "range_default")]
    range: f32,
    #[serde(default = "num_default")]
    num: u64,
}

fn range_default() -> f32 {
    0.0
}

fn num_default() -> u64 {
    1
}

pub struct Action;

impl<'a> System<'a> for Action {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, Context>,
        Read<'a, AnimationResource>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Tag>,
    );

    fn run(&mut self, (e, lazy, context, animations, pos, tag): Self::SystemData) {
        for (pos, tag) in (&pos, &tag).join() {
            if context.count % tag.frequency != 0 {
                continue;
            }

            let animation = animations.get(&tag.animation);

            for i in 0..tag.num {
                let (vx, vy) = if tag.num > 1 {
                    let r = tag.range / ((tag.num - 1) as f32) * (i as f32) - tag.range / 2.0
                        + 3.14 / 2.0;
                    let vx = r.cos() * tag.vel;
                    let vy = r.sin() * tag.vel;
                    (vx, vy)
                } else {
                    (0.0, tag.vel)
                };

                let mut pos = pos.clone();
                pos.x += pos.w / 2.0 - tag.size.0 / 2.0;
                pos.y += pos.h;
                pos.w = tag.size.0;
                pos.h = tag.size.1;
                lazy.create_entity(&e)
                    .with(pos)
                    .with(Bullet::enemy(tag.damage))
                    .with(animation.clone())
                    .with(Lifetime::Frameout)
                    .with(Vel::new(vx, vy))
                    .build();
            }
        }
    }
}
