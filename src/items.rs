use crate::{components::*, resources::*, utils::*};
use specs::prelude::*;

pub fn spawn<'a>(world: &mut World) {
    let count = world.fetch::<Context>().count;

    if count % 600 != 0 {
        return;
    }

    world
        .create_entity()
        .with(Pos::new(random_x(), 0.0, 0.0, 20.0, 20.0))
        .with(Vel::new(0.0, 0.4))
        .with(Item::new(1))
        .with(Animation::new(AssetId::new("power_a1"), 20).add(AssetId::new("power_a2"), 20))
        .with(Lifetime::Frameout)
        .build();
}
