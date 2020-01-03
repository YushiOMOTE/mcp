use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_yaml::Value;
use specs::prelude::*;

fn parse<T: DeserializeOwned>(value: Value) -> T {
    serde_yaml::from_value(value).expect("Couldn't parse config")
}

#[derive(Debug, Clone, Deserialize)]
pub struct FeatureConfig {
    name: String,
    #[serde(flatten)]
    value: Value,
}

macro_rules! features {
    ($($name:tt),*) => {
        $(pub mod $name;)*

        pub fn init(world: &mut World) {
            $(world.register::<$name::Tag>();)*
        }

        pub fn update(world: &mut World) {
            $($name::Action.run_now(world);)*
        }

        pub fn setup<B: Builder>(builder: B, cfg: FeatureConfig) -> B {
            match cfg.name.as_ref() {
                $(stringify!($name) => builder.with(parse::<$name::Tag>(cfg.value)),)*
                _ => panic!("No such motion name: {}", cfg.name),
            }
        }
    }
}

features! {
    linear_move,
    wave_move,
    radial_attack,
    control,
    shooter
}
