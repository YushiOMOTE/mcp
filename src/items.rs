use crate::{components::*, utils::*};
use specs::prelude::*;

pub fn items_spawn<'a>(world: &mut World, count: u64) {
    if count % 200 != 0 {
        return;
    }

    world
        .create_entity()
        .with(Pos::new(random_x(), 0.0, 0.0, 20.0, 20.0))
        .with(Vel::new(0.0, 0.4))
        .with(Item::new(1))
        .with(Animation::new(AssetId::new(9), 20).add(AssetId::new(10009), 20))
        .with(Lifetime::Frameout)
        .build();
}
