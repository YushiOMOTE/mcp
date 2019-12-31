use crate::{assets::*, components::*, resources::*};
use quicksilver::prelude::*;
use specs::prelude::*;
use specs::world::LazyBuilder;

pub struct ActionBomb<'a, 'b> {
    window: &'a mut Window,
    assets: &'b mut Assets,
}

impl<'a, 'b> ActionBomb<'a, 'b> {
    pub fn new(window: &'a mut Window, assets: &'b mut Assets) -> Self {
        Self { window, assets }
    }
}

impl<'a, 'b, 'c> System<'a> for ActionBomb<'b, 'c> {
    type SystemData = (Entities<'a>, ReadStorage<'a, Pos>, WriteStorage<'a, Bomb>);

    fn run(&mut self, (e, pos, mut bomb): Self::SystemData) {
        for (be, pos, bomb) in (&e, &pos, &mut bomb).join() {
            let off = (bomb.counter / 4) * 10000;
            self.assets
                .draw(self.window, &AssetId::new(off + 6), pos, 0);
            bomb.counter += 1;

            if bomb.counter >= 16 {
                let _ = e.delete(be);
            }
        }
    }
}

pub fn create_bomb<'a>(e: LazyBuilder<'a>, pos: &Pos) {
    let mut pos = pos.clone();
    pos.w = 20.0;
    pos.h = 20.0;
    e.with(Bomb::new()).with(pos).build();
}
