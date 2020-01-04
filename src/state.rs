use crate::{
    animations,
    assets::*,
    background,
    components::{self, *},
    entities, extra,
    resources::*,
    scenarios, systems,
};
use quicksilver::{
    graphics::Color,
    input::{ButtonState, Key},
    lifecycle::{Event, State, Window},
    Result,
};
use specs::prelude::*;

components! {
    Components {
        components::Enemy = "enemy",
        components::Item = "item",
        components::Player = "player",
        components::Bound = "bound",
        components::MustLive = "mustlive",
        extra::linear_move::Tag = "linear_move",
        extra::wave_move::Tag = "wave_move",
        extra::radial_attack::Tag = "radial_attack",
        extra::control::Tag = "control",
        extra::shooter::Tag = "shooter",
    }
}

systems! {
    Systems {
        systems::MoveObjects,
        systems::MaintainLifetime,
        systems::BulletCollisions,
        systems::EnemyCollisions,
        systems::ItemCollisions,
        systems::CheckGameOver,
        extra::linear_move::Action,
        extra::wave_move::Action,
        extra::radial_attack::Action,
        extra::control::Action,
        extra::shooter::Action,
    }
}

pub struct Play {
    world: World,
    assets: Option<Assets>,
}

impl State for Play {
    fn new() -> Result<Self> {
        let mut world = World::new();

        world.insert(Context::new());
        world.insert(Events::new());
        world.insert(animations::AnimationConfig::from_static_file());
        world.insert(entities::EntitiesConfig::from_static_file());
        world.insert(scenarios::ScenarioConfig::from_static_file());
        world.register::<Vel>();
        world.register::<Pos>();
        world.register::<Bound>();
        world.register::<Lifetime>();
        world.register::<Animation>();
        world.register::<Player>();
        world.register::<Enemy>();
        world.register::<Bullet>();
        world.register::<Item>();
        world.register::<MustLive>();

        Components.register(&mut world);

        background::spawn(&mut world);
        scenarios::spawn(&mut world);

        let asset_cfg = AssetsConfig::from_static_file();

        Ok(Self {
            world,
            assets: Some(Assets::new(&asset_cfg)),
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        self.world.fetch_mut::<Events>().push(event.clone());

        match *event {
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        }

        Ok(())
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        Systems.run_now(&mut self.world);
        self.world.fetch_mut::<Events>().clear();
        self.world.maintain();

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.world.fetch_mut::<Context>().update();

        scenarios::spawn(&mut self.world);

        {
            if self.world.fetch::<Context>().gameover {
                window.clear(Color::RED)?;
                return Ok(());
            } else {
                window.clear(Color::BLACK)?;
            }
        }

        {
            let mut assets = self.assets.take().unwrap();
            systems::DrawObjects::new(window, &mut assets).run_now(&mut self.world);
            self.assets = Some(assets);
        }

        Ok(())
    }
}
