use crate::{components::*, resources::*};
use specs::prelude::*;

pub fn background_spawn<'a>(world: &mut World) {
    world
        .create_entity()
        .with(Pos::new(0.0, -HEIGHT * 2.0, -1.0, WIDTH, HEIGHT * 2.0))
        .with(Vel::new(0.0, 0.4))
        .with(Animation::new(AssetId::new(7), 1))
        .with(Lifetime::Scroll(HEIGHT * 2.0))
        .build();
    world
        .create_entity()
        .with(Pos::new(0.0, 0.0, -1.0, WIDTH, HEIGHT * 2.0))
        .with(Vel::new(0.0, 0.4))
        .with(Animation::new(AssetId::new(7), 1))
        .with(Lifetime::Scroll(HEIGHT * 2.0))
        .build();
}
