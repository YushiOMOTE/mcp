use crate::{components::*, resources::*};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Component)]
pub struct Label;

pub fn spawn(world: &mut World, x: f32, y: f32) {
    let animation = Animation::new(AssetId::new(10), 10).add(AssetId::new(10010), 10);

    world
        .create_entity()
        .with(Pos::new(x, y, 0.0, 24.0, 24.0))
        .with(Enemy::new(1))
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
        if context.count % 80 != 0 {
            return;
        }

        for (pos, _) in (&pos, &normal).join() {
            let animation = Animation::new(AssetId::new(3), 10).add(AssetId::new(10003), 10);

            let mut pos = pos.clone();
            pos.x += 5.0;
            pos.w = 10.0;
            pos.h = 10.0;
            lazy.create_entity(&e)
                .with(pos)
                .with(Bullet::enemy(10))
                .with(animation)
                .with(Lifetime::Frameout)
                .with(Vel::new(0.0, 2.0))
                .build();
        }
    }
}
