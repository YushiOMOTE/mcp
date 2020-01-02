use super::*;

macro_rules! motion {
    ($($name:tt),*) => {
        $(mod $name;)*

        pub fn init(world: &mut World) {
           $(world.register::<$name::Label>();)*
        }

        pub fn update(world: &mut World) {
           $($name::Action.run_now(world);)*
        }

        pub fn setup<B: Builder>(builder: B, cfg: &MotionConfig) -> B {
            match cfg.name.as_ref() {
                $(stringify!($name) => builder.with($name::Label::new(cfg.clone())),)*
                    _ => panic!("No such motion name: {}", cfg.name),
            }
        }
    }
}

motion! {
    constant,
    wave
}
