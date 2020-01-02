use crate::{components::*, enemies::*, resources::*};
use derive_new::new;
use specs::prelude::*;
use specs_derive::Component;

#[derive(new, Debug, Component)]
pub struct Label {
    cfg: MotionConfig,
}

pub struct Action;

impl<'a> System<'a> for Action {
    type SystemData = (WriteStorage<'a, Vel>, ReadStorage<'a, Label>);

    fn run(&mut self, (mut vel, label): Self::SystemData) {
        for (mut vel, _label) in (&mut vel, &label).join() {
            vel.x = 0.0;
            vel.y = 0.4;
        }
    }
}
