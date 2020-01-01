use crate::components::*;
use specs::prelude::*;
use specs::world::LazyBuilder;

pub fn bomb_spawn<'a>(e: LazyBuilder<'a>, pos: &Pos) {
    let animation = Animation::new(AssetId::new(6), 5)
        .add(AssetId::new(10006), 5)
        .add(AssetId::new(20006), 5)
        .add(AssetId::new(30006), 5);
    let mut pos = pos.clone();
    pos.w = 20.0;
    pos.h = 20.0;
    e.with(animation)
        .with(pos)
        .with(Lifetime::Timer(20))
        .build();
}
