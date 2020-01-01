use crate::{resources::*, utils::*};
use specs::prelude::*;

macro_rules! enemy {
    ($($id:tt),*) => {
        $(mod $id;)*

        pub fn init(world: &mut World) {
            $(world.register::<$id::Label>();)*
        }

        pub fn update(world: &mut World) {
            $($id::Action.run_now(world);)*
        }

        pub fn spawn_one(world: &mut World, id: &str, x: f32, y: f32) {
            match id {
                $(stringify!($id) => $id::spawn(world, x, y),)*
                _ => panic!("No such enemy id: {}", id),
            }
        }
    };
}

enemy! {
    boss1,
    normal1,
    normal2,
    normal3
}

pub fn spawn(world: &mut World) {
    let count = world.fetch::<Context>().count;

    let num = match count {
        0..=1000 => 400,
        1001..=2000 => 300,
        2001..=3000 => 200,
        3001..=4000 => 100,
        4001..=5000 => 400,
        5001..=6000 => 100,
        6001..=7000 => 50,
        7001..=8000 => 400,
        8001..=9000 => 20,
        9001..=10000 => 400,
        _ => 800,
    };

    if count % num == 0 {
        match (count / 100) % 3 {
            0 => spawn_one(world, "normal1", random_x(), 0.0),
            1 => spawn_one(world, "normal2", random_x(), 0.0),
            2 => spawn_one(world, "normal3", random_x(), 0.0),
            _ => unreachable!(),
        }
    }

    if count == 4000 {
        spawn_one(world, "boss1", WIDTH / 2.0 - 100.0, -200.0);
    }

    if count == 7000 {
        spawn_one(world, "boss1", WIDTH / 2.0 - 400.0, -200.0);
        spawn_one(world, "boss1", WIDTH / 2.0 + 300.0, -200.0);
    }

    if count == 9000 {
        spawn_one(world, "boss1", WIDTH / 2.0 - 400.0, -200.0);
        spawn_one(world, "boss1", WIDTH / 2.0 - 100.0, -200.0);
        spawn_one(world, "boss1", WIDTH / 2.0 + 300.0, -200.0);
    }
}
