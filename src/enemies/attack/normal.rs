use crate::{components::*, enemies::*};
use serde::Deserialize;
use specs_derive::Component;

#[derive(Debug, Deserialize)]
struct Config {
    vel: f32,
    #[serde(default = "range_default")]
    range: f32,
    #[serde(default = "count_default")]
    count: u64,
}

fn range_default() -> f32 {
    0.0
}

fn count_default() -> u64 {
    1
}

#[derive(Debug, Component)]
pub struct Label {
    cfg: AttackConfig,
    params: Config,
}

impl Label {
    pub fn new(cfg: AttackConfig) -> Self {
        Self {
            params: parse(&cfg.params),
            cfg: cfg,
        }
    }
}

pub struct Action;

impl<'a> System<'a> for Action {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, Context>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Label>,
    );

    fn run(&mut self, (e, lazy, context, pos, label): Self::SystemData) {
        for (pos, label) in (&pos, &label).join() {
            if context.count % label.cfg.frequency != 0 {
                continue;
            }

            let animation = label.cfg.animation.iter().fold(Animation::empty(), |a, f| {
                a.add(AssetId::new(f.aid), f.time)
            });

            let p = &label.params;

            for i in 0..p.count {
                let (vx, vy) = if p.count > 1 {
                    let r =
                        p.range / ((p.count - 1) as f32) * (i as f32) - p.range / 2.0 + 3.14 / 2.0;
                    let vx = r.cos() * p.vel;
                    let vy = r.sin() * p.vel;
                    (vx, vy)
                } else {
                    (0.0, p.vel)
                };

                let mut pos = pos.clone();
                pos.x += pos.w / 2.0 - label.cfg.size.0 / 2.0;
                pos.y += pos.h;
                pos.w = label.cfg.size.0;
                pos.h = label.cfg.size.1;
                lazy.create_entity(&e)
                    .with(pos)
                    .with(Bullet::enemy(label.cfg.damage))
                    .with(animation.clone())
                    .with(Lifetime::Frameout)
                    .with(Vel::new(vx, vy))
                    .build();
            }
        }
    }
}
