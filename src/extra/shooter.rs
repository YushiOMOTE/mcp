use crate::{components::*, resources::*};
use quicksilver::prelude::*;
use serde::Deserialize;
use specs::{prelude::*, world::EntitiesRes};
use specs_derive::Component;

#[derive(Debug, Deserialize, Component)]
pub struct Tag {}

pub struct Action;

impl Action {
    fn shoot(&self, entities: &EntitiesRes, lazy: &LazyUpdate, pos: &Pos, player: &Player) {
        let mut pos = pos.clone();

        match player.level {
            0..=3 => {
                pos.x += 6.0;
                pos.w = 6.0;
                pos.h = 14.0;

                lazy.create_entity(entities)
                    .with(pos)
                    .with(
                        Animation::new(AssetId::new("shot_a1"), 10)
                            .add(AssetId::new("shot_a2"), 10),
                    )
                    .with(Vel::new(0.0, -10.0))
                    .with(Bullet::player(8))
                    .with(Lifetime::Frameout)
                    .build();
            }
            4..=5 => {
                for i in 0..3 {
                    let mut pos = pos.clone();
                    pos.x += (i as f32) * 16.0 - 12.0;
                    pos.w = 6.0;
                    pos.h = 14.0;

                    lazy.create_entity(entities)
                        .with(pos)
                        .with(
                            Animation::new(AssetId::new("shot_a1"), 10)
                                .add(AssetId::new("shot_a2"), 10),
                        )
                        .with(Vel::new(0.0, -10.0))
                        .with(Bullet::player(5))
                        .with(Lifetime::Frameout)
                        .build();
                }
            }
            6..=7 => {
                for i in 0..5 {
                    let mut pos = pos.clone();
                    pos.x += (i as f32) * 12.0 - 18.0;
                    pos.w = 6.0;
                    pos.h = 14.0;

                    lazy.create_entity(entities)
                        .with(pos)
                        .with(
                            Animation::new(AssetId::new("shot_a1"), 10)
                                .add(AssetId::new("shot_a2"), 10),
                        )
                        .with(Vel::new(0.0, -10.0))
                        .with(Bullet::player(4))
                        .with(Lifetime::Frameout)
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

                    lazy.create_entity(entities)
                        .with(pos)
                        .with(
                            Animation::new(AssetId::new("shot_a1"), 10)
                                .add(AssetId::new("shot_a2"), 10),
                        )
                        .with(Vel::new(sx, sy))
                        .with(Bullet::player(4))
                        .with(Lifetime::Frameout)
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

                    lazy.create_entity(entities)
                        .with(pos)
                        .with(
                            Animation::new(AssetId::new("shot_a1"), 10)
                                .add(AssetId::new("shot_a2"), 10),
                        )
                        .with(Vel::new(sx, sy))
                        .with(Bullet::player(4))
                        .with(Lifetime::Frameout)
                        .build();
                }
            }
        }
    }
}

impl<'a> System<'a> for Action {
    type SystemData = (
        Read<'a, Events>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Tag>,
    );

    fn run(&mut self, (events, entities, lazy, pos, player, tag): Self::SystemData) {
        for event in events.get() {
            for (pos, player, _tag) in (&pos, &player, &tag).join() {
                match event {
                    Event::Key(Key::Z, ButtonState::Pressed) => {
                        self.shoot(&*entities, &*lazy, pos, player)
                    }
                    _ => {}
                }
            }
        }
    }
}
