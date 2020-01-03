use crate::{components::*, resources::*};
use quicksilver::prelude::*;
use serde::Deserialize;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Deserialize, Component)]
pub struct Tag {
    pub vel: f32,
}

pub struct Action;

impl<'a> System<'a> for Action {
    type SystemData = (
        Read<'a, Events>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Tag>,
    );

    fn run(&mut self, (events, mut vel, tag): Self::SystemData) {
        for event in events.get() {
            for (mut vel, tag) in (&mut vel, &tag).join() {
                match event {
                    Event::Key(Key::Left, ButtonState::Pressed) => {
                        vel.x = -tag.vel;
                    }
                    Event::Key(Key::Right, ButtonState::Pressed) => {
                        vel.x = tag.vel;
                    }
                    Event::Key(Key::Up, ButtonState::Pressed) => {
                        vel.y = -tag.vel;
                    }
                    Event::Key(Key::Down, ButtonState::Pressed) => {
                        vel.y = tag.vel;
                    }
                    Event::Key(Key::Left, ButtonState::Released) => {
                        vel.x = 0.0;
                    }
                    Event::Key(Key::Right, ButtonState::Released) => {
                        vel.x = 0.0;
                    }
                    Event::Key(Key::Up, ButtonState::Released) => {
                        vel.y = 0.0;
                    }
                    Event::Key(Key::Down, ButtonState::Released) => {
                        vel.y = 0.0;
                    }
                    _ => {}
                }
            }
        }
    }
}
