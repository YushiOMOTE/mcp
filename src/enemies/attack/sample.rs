use crate::{components::*, enemies::*, resources::*};
use derive_new::new;
use specs::prelude::*;
use specs_derive::Component;

#[derive(new, Debug, Component)]
pub struct Label {
    cfg: AttackConfig,
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

            for i in 0..=8 {
                let r = (3.14 / 8.0) * (i as f32) - 3.14 / 2.0;
                let vx = r.sin() * 2.0;
                let vy = r.cos() * 2.0;
                let animation = Animation::new(AssetId::new(3), 10).add(AssetId::new(10003), 10);

                let mut pos = pos.clone();
                pos.x += 10.0;
                pos.y += 30.0;
                pos.w = 10.0;
                pos.h = 10.0;
                lazy.create_entity(&e)
                    .with(pos)
                    .with(Bullet::enemy(10))
                    .with(animation)
                    .with(Lifetime::Frameout)
                    .with(Vel::new(vx, vy))
                    .build();
            }
        }
    }
}
