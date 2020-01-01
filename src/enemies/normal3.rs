use crate::{components::*, resources::*};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Component)]
pub struct Label;

pub fn spawn(world: &mut World, x: f32, y: f32) {
    let animation = Animation::new(AssetId::new(11), 10).add(AssetId::new(10011), 10);

    world
        .create_entity()
        .with(Pos::new(x, y, 0.0, 26.0, 30.0))
        .with(Enemy::new(300))
        .with(animation)
        .with(Vel::new(0.0, 1.0))
        .with(Label)
        .with(Lifetime::Frameout)
        .build();
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

    fn run(&mut self, (e, lazy, context, pos, normal): Self::SystemData) {
        if context.count % 150 != 0 {
            return;
        }

        for (pos, _) in (&pos, &normal).join() {
            for i in 0..=2 {
                let r = (3.14 / 2.0) * (i as f32) + 3.14 / 2.0;
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
