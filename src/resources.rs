use crate::components::*;
use specs::prelude::*;

pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 600.0;

#[derive(Default, Clone)]
pub struct Counter {
    pub count: u64,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }
}

#[derive(Default, Clone)]
pub struct Player {
    pub life: u64,
    pub pos: Pos,
    pub asset: AssetId,
}

impl Player {
    pub fn new() -> Self {
        Self {
            life: 100,
            pos: Pos::new(WIDTH / 2.0, HEIGHT * 0.9, 15.0, 30.0),
            asset: AssetId::new(0),
        }
    }

    pub fn move_left(&mut self) {
        self.pos.x = (self.pos.x - 15.0).max(0.0);
    }

    pub fn move_right(&mut self) {
        self.pos.x = (self.pos.x + 15.0).min(WIDTH - self.pos.w);
    }

    pub fn move_up(&mut self) {
        self.pos.y = (self.pos.y - 15.0).max(0.0);
    }

    pub fn move_down(&mut self) {
        self.pos.y = (self.pos.y + 15.0).min(HEIGHT - self.pos.h);
    }

    pub fn shoot(&self, world: &mut World) {
        let mut pos = self.pos.clone();
        pos.x += 6.0;
        pos.w = 6.0;
        pos.h = 14.0;

        world
            .create_entity()
            .with(pos)
            .with(AssetId::new(1))
            .with(Vel::new(0.0, -10.0))
            .with(Bullet::player(10))
            .build();
    }
}
