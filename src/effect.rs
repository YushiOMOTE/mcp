use crate::components::*;
use specs::prelude::*;
use specs::world::LazyBuilder;

pub fn bomb_spawn<'a>(e: LazyBuilder<'a>, pos: &Pos) {
    let animation = Animation::new(AssetId::new("explosion_a1"), 5)
        .add(AssetId::new("explosion_a2"), 5)
        .add(AssetId::new("explosion_a3"), 5)
        .add(AssetId::new("explosion_a4"), 5);
    let mut pos = pos.clone();
    pos.z = 1.0;
    pos.w = 20.0;
    pos.h = 20.0;
    e.with(animation)
        .with(pos)
        .with(Lifetime::Timer(20))
        .build();
}
