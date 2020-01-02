use crate::{components::*, resources::*};
use serde::Deserialize;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Deserialize, Component)]
pub struct Tag {
    width: f32,
    freq: u64,
    count: Option<u64>,
}

pub struct Action;

impl<'a> System<'a> for Action {
    type SystemData = (
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Tag>,
        Read<'a, Context>,
    );

    fn run(&mut self, (mut pos, mut tag, ctx): Self::SystemData) {
        for (mut pos, tag) in (&mut pos, &mut tag).join() {
            let count = *tag.count.get_or_insert_with(|| ctx.count);
            let count = ctx.count - count;

            let r = 6.28 * ((count % tag.freq) as f32) / (tag.freq as f32);
            pos.x += r.cos() * tag.width;
        }
    }
}
