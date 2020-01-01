use crate::{assets::*, components::*};
use quicksilver::prelude::*;
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

    pos.x += 6.0;
    pos.w = 6.0;
    pos.h = 14.0;

    world
        .create_entity()
        .with(pos)
        .with(Animation::new(AssetId::new(1), 10).add(AssetId::new(10001), 10))
        .with(Vel::new(0.0, -10.0))
        .with(Bullet::player(10))
        .build();
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
