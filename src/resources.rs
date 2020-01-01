use crate::components::*;
use specs::prelude::*;

pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 600.0;

#[derive(Default, Clone)]
pub struct Context {
    pub count: u64,
}

impl Context {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn update(&mut self) {
        self.count += 1;
    }
}

#[derive(Default, Clone)]
pub struct User {
    pub entity: Option<Entity>,
}

impl User {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
        }
    }
}

pub fn user_spawn(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Pos::new(WIDTH / 2.0, HEIGHT * 0.9, 0.0, 16.0, 24.0))
        .with(Player::new(150, 0))
        .with(Vel::new(0.0, 0.0))
        .with(Bound::new(0.0, 0.0, WIDTH, HEIGHT))
        .with(Animation::new(AssetId::new(0), 10).add(AssetId::new(10000), 10))
        .build()
}

pub fn user_clear_y(world: &mut World) {
    update_user_vel(world, |mut vel| {
        vel.y = 0.0;
    });
}

pub fn user_clear_x(world: &mut World) {
    update_user_vel(world, |mut vel| {
        vel.x = 0.0;
    });
}

pub fn user_shoot(world: &mut World) {
    let mut pos = match user_pos(world) {
        Some(p) => p,
        None => return,
    };

    let player = match user_player(world) {
        Some(p) => p,
        None => return,
    };

    match player.level {
        0..=3 => {
            pos.x += 6.0;
            pos.w = 6.0;
            pos.h = 14.0;

            world
                .create_entity()
                .with(pos)
                .with(Animation::new(AssetId::new(1), 10).add(AssetId::new(10001), 10))
                .with(Vel::new(0.0, -10.0))
                .with(Bullet::player(8))
                .build();
        }
        4..=5 => {
            for i in 0..3 {
                let mut pos = pos.clone();
                pos.x += (i as f32) * 16.0 - 12.0;
                pos.w = 6.0;
                pos.h = 14.0;

                world
                    .create_entity()
                    .with(pos)
                    .with(Animation::new(AssetId::new(1), 10).add(AssetId::new(10001), 10))
                    .with(Vel::new(0.0, -10.0))
                    .with(Bullet::player(5))
                    .build();
            }
        }
        6..=7 => {
            for i in 0..5 {
                let mut pos = pos.clone();
                pos.x += (i as f32) * 12.0 - 18.0;
                pos.w = 6.0;
                pos.h = 14.0;

                world
                    .create_entity()
                    .with(pos)
                    .with(Animation::new(AssetId::new(1), 10).add(AssetId::new(10001), 10))
                    .with(Vel::new(0.0, -10.0))
                    .with(Bullet::player(4))
                    .build();
            }
        }
        8..=9 => {
            for i in 0..10 {
                let mut pos = pos.clone();
                pos.x += 6.0;
                pos.w = 6.0;
                pos.h = 14.0;

                let s = (3.14 / 2.0 / 9.0) * (i as f32) + 3.14 / 2.0 + 3.14 / 4.0;
                let sx = s.sin() * 10.0;
                let sy = s.cos() * 10.0;

                world
                    .create_entity()
                    .with(pos)
                    .with(Animation::new(AssetId::new(1), 10).add(AssetId::new(10001), 10))
                    .with(Vel::new(sx, sy))
                    .with(Bullet::player(4))
                    .build();
            }
        }
        _ => {
            for i in 0..25 {
                let mut pos = pos.clone();
                pos.x += 6.0;
                pos.w = 6.0;
                pos.h = 14.0;

                let s = (3.14 / 24.0) * (i as f32) + 3.14 / 2.0;
                let sx = s.sin() * 10.0;
                let sy = s.cos() * 10.0;

                world
                    .create_entity()
                    .with(pos)
                    .with(Animation::new(AssetId::new(1), 10).add(AssetId::new(10001), 10))
                    .with(Vel::new(sx, sy))
                    .with(Bullet::player(4))
                    .build();
            }
        }
    }
}

pub fn user_alive(world: &mut World) -> bool {
    let entity = world.fetch::<User>().entity.unwrap();
    world.is_alive(entity)
}

pub fn user_pos(world: &mut World) -> Option<Pos> {
    let user = world.fetch_mut::<User>();
    let entity = user.entity?;
    let mut pos = world.write_storage::<Pos>();
    pos.get_mut(entity).cloned()
}

pub fn user_player(world: &mut World) -> Option<Player> {
    let user = world.fetch_mut::<User>();
    let entity = user.entity?;
    let player = world.read_storage::<Player>();
    player.get(entity).cloned()
}

#[allow(unused)]
pub fn user_vel(world: &mut World) -> Option<Vel> {
    let user = world.fetch_mut::<User>();
    let entity = user.entity?;
    let mut vel = world.write_storage::<Vel>();
    vel.get_mut(entity).cloned()
}

pub fn update_user_vel<F: FnOnce(&mut Vel)>(world: &mut World, f: F) {
    let user = world.fetch_mut::<User>();
    let entity = match user.entity {
        Some(e) => e,
        None => return,
    };

    let mut vel = world.write_storage::<Vel>();
    let mut vel = match vel.get_mut(entity) {
        Some(p) => p,
        None => return,
    };

    f(&mut vel);
}

pub fn user_move_left(world: &mut World) {
    update_user_vel(world, |vel| {
        vel.x = -3.0;
    });
}

pub fn user_move_right(world: &mut World) {
    update_user_vel(world, |vel| {
        vel.x = 3.0;
    });
}

pub fn user_move_up(world: &mut World) {
    update_user_vel(world, |vel| {
        vel.y = -3.0;
    });
}

pub fn user_move_down(world: &mut World) {
    update_user_vel(world, |vel| {
        vel.y = 3.0;
    });
}
