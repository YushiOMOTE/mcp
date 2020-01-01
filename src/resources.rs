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

pub fn user_alive(world: &mut World) -> bool {
    let entity = world.fetch::<User>().entity.unwrap();
    world.is_alive(entity)
}

pub fn user_pos(world: &mut World) -> Pos {
    let user = world.fetch_mut::<User>();
    let entity = user.entity.unwrap();
    let mut pos = world.write_storage::<Pos>();
    pos.get_mut(entity).unwrap().clone()
}

pub fn update_user_pos<F: FnOnce(&mut Pos)>(world: &mut World, f: F) {
    let user = world.fetch_mut::<User>();
    let entity = match user.entity {
        Some(e) => e,
        None => return,
    };

    let mut pos = world.write_storage::<Pos>();
    let mut pos = match pos.get_mut(entity) {
        Some(p) => p,
        None => return,
    };

    f(&mut pos);
}

pub fn user_move_left(world: &mut World) {
    update_user_pos(world, |pos| {
        pos.x = (pos.x - 15.0).max(0.0);
    });
}

pub fn user_move_right(world: &mut World) {
    update_user_pos(world, |pos| {
        pos.x = (pos.x + 15.0).min(WIDTH - pos.w);
    });
}

pub fn user_move_up(world: &mut World) {
    update_user_pos(world, |pos| {
        pos.y = (pos.y - 15.0).max(0.0);
    });
}

pub fn user_move_down(world: &mut World) {
    update_user_pos(world, |pos| {
        pos.y = (pos.y + 15.0).min(HEIGHT - pos.h);
    });
}
