use super::*;
use crate::{components::*, resources::*};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Component)]
pub struct Label;

pub fn spawn(world: &mut World, x: f32, y: f32) {
    let animation = Animation::new(AssetId::new(5), 1);

    world
        .create_entity()
        .with(Pos::new(x, y, 0.0, 200.0, 200.0))
        .with(Enemy::new(500))
        .with(animation)
        .with(Lifetime::Frameout)
        .with(Vel::new(0.0, 0.3))
        .with(Label)
        .build();
}

pub fn init(world: &mut World) {
    world.register::<Label>();
}

pub fn setup<B: Builder>(builder: B, cfg: &MotionConfig) -> B {
    builder.with(Label)
}

pub fn update(world: &mut World) {
    Action.run_now(world);
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

    fn run(&mut self, (e, lazy, context, pos, boss): Self::SystemData) {
        if context.count % 40 != 0 {
            return;
        }

        for (pos, _) in (&pos, &boss).join() {
            for i in 0..32 {
                let animation = Animation::new(AssetId::new(3), 10).add(AssetId::new(10003), 10);

                let r = (3.14 * 2.0 / 32.0) * (i as f32);
                let vx = r.sin() * 2.0;
                let vy = r.cos() * 2.0;
                let mut pos = pos.clone();
                pos.x += pos.w / 2.0 + (((context.count / 40) % 8) as f32 * 20.0) - 80.0;
                pos.y += pos.h / 2.0;
                pos.w = 10.0;
                pos.h = 10.0;
                lazy.create_entity(&e)
                    .with(pos)
                    .with(Bullet::enemy(10))
                    .with(animation)
                    .with(Vel::new(vx, vy))
                    .with(Lifetime::Frameout)
                    .build();
            }
        }
    }
}
