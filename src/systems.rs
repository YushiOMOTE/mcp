use crate::{assets::*, components::*, effect::*, resources::*};
use quicksilver::prelude::*;
use specs::prelude::*;

pub struct MoveObjects;

impl<'a> System<'a> for MoveObjects {
    type SystemData = (
        WriteStorage<'a, Pos>,
        ReadStorage<'a, Vel>,
        ReadStorage<'a, Bound>,
    );

    fn run(&mut self, (mut pos, vel, bound): Self::SystemData) {
        for (pos, vel, bound) in (&mut pos, &vel, bound.maybe()).join() {
            pos.x += vel.x;
            pos.y += vel.y;

            if let Some(b) = bound {
                pos.x = pos.x.max(b.x).min(b.x + b.w - pos.w);
                pos.y = pos.y.max(b.y).min(b.y + b.h - pos.h);
            }
        }
    }
}

pub struct DrawObjects<'a, 'b> {
    window: &'a mut Window,
    assets: &'b mut Assets,
}

impl<'a, 'b> DrawObjects<'a, 'b> {
    pub fn new(window: &'a mut Window, assets: &'b mut Assets) -> Self {
        Self { window, assets }
    }
}

impl<'a, 'b, 'c> System<'a> for DrawObjects<'b, 'c> {
    type SystemData = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Animation>,
        Read<'a, Context>,
    );

    fn run(&mut self, (pos, animation, context): Self::SystemData) {
        for (pos, an) in (&pos, &animation).join() {
            self.assets.draw(self.window, an, pos, context.count);
        }
    }
}

pub struct MaintainLifetime;

impl<'a> System<'a> for MaintainLifetime {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Lifetime>,
    );

    fn run(&mut self, (e, mut pos, mut life): Self::SystemData) {
        for (ee, mut pos, mut life) in (&e, (&mut pos).maybe(), &mut life).join() {
            match &mut life {
                Lifetime::Frameout => {
                    let sw = WIDTH;
                    let sh = HEIGHT;

                    let m = 1.0;

                    if let Some(pos) = pos {
                        if (pos.x > sw * (1.0 + m) || pos.x < sw * -1.0 * m)
                            || (pos.y > sh * (1.0 + m) || pos.y < sh * -1.0 * m)
                        {
                            let _ = e.delete(ee);
                        }
                    }
                }
                Lifetime::Timer(count) => {
                    *count = count.saturating_sub(1);

                    if *count == 0 {
                        let _ = e.delete(ee);
                    }
                }
                Lifetime::Scroll(y) => {
                    if let Some(mut pos) = pos.as_mut() {
                        if pos.y >= *y {
                            pos.y = -1.0 * (*y);
                        }
                    }
                }
            }
        }
    }
}

fn hit(p1: &Pos, p2: &Pos) -> bool {
    !((p1.x + p1.w < p2.x || p2.x + p2.w < p1.x) || (p1.y + p1.h < p2.y || p2.y + p2.h < p1.y))
}

pub struct BulletCollisions;

impl<'a> System<'a> for BulletCollisions {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Bullet>,
        Read<'a, LazyUpdate>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Enemy>,
    );

    fn run(&mut self, (e, pos, bul, lazy, mut player, mut enemy): Self::SystemData) {
        for (be, bpos, bul) in (&e, &pos, &bul).join() {
            if bul.player {
                for (ee, epos, mut enemy) in (&e, &pos, &mut enemy).join() {
                    if hit(bpos, epos) {
                        enemy.life = enemy.life.saturating_sub(bul.damage);

                        if enemy.life == 0 {
                            println!("Enemy {:?} dies", ee);

                            let _ = e.delete(ee);
                        }

                        bomb_spawn(lazy.create_entity(&e), bpos);

                        let _ = e.delete(be);
                    }
                }
            } else {
                for (pe, ppos, mut player) in (&e, &pos, &mut player).join() {
                    if hit(bpos, ppos) {
                        player.life = player.life.saturating_sub(bul.damage);

                        if player.life == 0 {
                            println!("Player {:?} dies", pe);

                            let _ = e.delete(pe);
                        }

                        bomb_spawn(lazy.create_entity(&e), ppos);

                        let _ = e.delete(be);
                    }
                }
            }
        }
    }
}

pub struct EnemyCollisions;

impl<'a> System<'a> for EnemyCollisions {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Pos>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (e, pos, mut player, enemy, lazy): Self::SystemData) {
        for (ee, epos, enemy) in (&e, &pos, &enemy).join() {
            for (pe, ppos, mut player) in (&e, &pos, &mut player).join() {
                if hit(epos, ppos) {
                    player.life = player.life.saturating_sub(enemy.life);

                    if player.life == 0 {
                        println!("Player {:?} dies", pe);

                        let _ = e.delete(pe);
                    }

                    bomb_spawn(lazy.create_entity(&e), epos);

                    let _ = e.delete(ee);
                }
            }
        }
    }
}

pub struct ItemCollisions;

impl<'a> System<'a> for ItemCollisions {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Pos>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Item>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (e, pos, mut player, item, _lazy): Self::SystemData) {
        for (ie, ipos, item) in (&e, &pos, &item).join() {
            for (_pe, ppos, mut player) in (&e, &pos, &mut player).join() {
                if hit(ipos, ppos) {
                    player.level += item.id;
                    let _ = e.delete(ie);
                }
            }
        }
    }
}
