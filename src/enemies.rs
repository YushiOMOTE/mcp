use crate::{components::*, resources::*};
use specs::prelude::*;
use specs_derive::Component;

fn random_x() -> f32 {
    (rand::random::<u64>() % (WIDTH as u64)) as f32
}

#[derive(Debug, Component)]
pub struct Normal;

pub fn spawn_normal(world: &mut World) {
    let y = 0.0;
    let x = random_x();

    world
        .create_entity()
        .with(Pos::new(x, y, 26.0, 30.0))
        .with(Enemy::new(10))
        .with(AssetId::new(2))
        .with(Vel::new(0.0, 1.0))
        .with(Normal)
        .build();
}

pub struct MoveNormal;

impl<'a> System<'a> for MoveNormal {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, Counter>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Normal>,
    );

    fn run(&mut self, (e, lazy, counter, pos, normal): Self::SystemData) {
        if counter.count % 200 != 0 {
            return;
        }

        for (pos, _) in (&pos, &normal).join() {
            for v in &[Vel::new(0.0, 2.0), Vel::new(1.4, 1.4), Vel::new(-1.4, 1.4)] {
                let mut pos = pos.clone();
                pos.x += 10.0;
                pos.w = 10.0;
                pos.h = 10.0;
                lazy.create_entity(&e)
                    .with(pos)
                    .with(Bullet::enemy(10))
                    .with(AssetId::new(3))
                    .with(v.clone())
                    .build();
            }
        }
    }
}

#[derive(Debug, Component)]
pub struct Boss;

pub fn spawn_boss(world: &mut World) {
    let y = 0.0;
    let x = WIDTH / 2.0 - 100.0;

    world
        .create_entity()
        .with(Pos::new(x, y, 200.0, 200.0))
        .with(Enemy::new(500))
        .with(AssetId::new(5))
        .with(Vel::new(0.0, 0.3))
        .with(Boss)
        .build();
}

pub struct MoveBoss;

impl<'a> System<'a> for MoveBoss {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, Counter>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Boss>,
    );

    fn run(&mut self, (e, lazy, counter, pos, boss): Self::SystemData) {
        if counter.count % 40 != 0 {
            return;
        }

        for (pos, _) in (&pos, &boss).join() {
            for i in 0..32 {
                let r = (3.14 * 2.0 / 32.0) * (i as f32);
                let vx = r.sin() * 2.0;
                let vy = r.cos() * 2.0;
                let mut pos = pos.clone();
                pos.x += pos.w / 2.0 + (((counter.count / 40) % 8) as f32 * 20.0) - 80.0;
                pos.y += pos.h / 2.0;
                pos.w = 10.0;
                pos.h = 10.0;
                lazy.create_entity(&e)
                    .with(pos)
                    .with(Bullet::enemy(10))
                    .with(AssetId::new(3))
                    .with(Vel::new(vx, vy))
                    .build();
            }
        }
    }
}

pub fn move_enemies(world: &mut World) {
    MoveBoss.run_now(world);
    MoveNormal.run_now(world);
}
