use serde::Deserialize;
use serde_yaml::Value;

pub mod control;
pub mod linear_move;
pub mod radial_attack;
pub mod shooter;
pub mod wave_move;

#[derive(Debug, Clone, Deserialize)]
pub struct ComponentConfig {
    pub name: String,
    #[serde(flatten)]
    pub value: Value,
}

macro_rules! components {
    ($id:ident { $($component:path = $name:literal,)* }) => {
        pub struct $id;

        impl $id {
            pub fn register(&self, world: &mut World) {
                $(world.register::<$component>();)*
            }

            pub fn setup<B: Builder>(&self, builder: B, cfg: crate::extra::ComponentConfig) -> B {
                match cfg.name.as_ref() {
                    $($name => builder.with(serde_yaml::from_value::<$component>(cfg.value).expect(&format!("Couldn't parse {}", $name))),)*
                    _ => panic!("No such component name: {}", cfg.name),
                }
            }
        }
    }
}

macro_rules! systems {
    ($id:ident { $($system:path,)* }) => {
        pub struct $id;

        impl $id {
            pub fn run_now(&self, world: &mut World) {
                $($system.run_now(world);)*
            }
        }
    }
}
