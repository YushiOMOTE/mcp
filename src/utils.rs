use crate::resources::*;

pub fn random_x() -> f32 {
    (rand::random::<u64>() % (WIDTH as u64)) as f32
}
