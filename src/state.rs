use crate::{
    animations, assets::*, background::*, components::*, enemies, features, items, resources::*,
    systems::*,
};
use quicksilver::{
    graphics::Color,
    input::{ButtonState, Key},
    lifecycle::{Event, State, Window},
    Result,
};
use specs::prelude::*;

pub struct Play {
    world: World,
    assets: Option<Assets>,
}

impl State for Play {
    fn new() -> Result<Self> {
        let mut world = World::new();

        world.insert(Context::new());
        world.insert(animations::AnimationResource::from_static_file());
        world.insert(UserConfig::from_static_file());
        world.insert(enemies::EnemiesConfig::from_static_file());
        world.register::<Vel>();
        world.register::<Pos>();
        world.register::<Bound>();
        world.register::<Lifetime>();
        world.register::<Animation>();
        world.register::<AssetId>();
        world.register::<Player>();
        world.register::<Enemy>();
        world.register::<Bullet>();
        world.register::<Item>();

        features::init(&mut world);

        background_spawn(&mut world);

        let user = User::new(&mut world);
        world.insert(user);

        let asset_cfg = AssetsConfig::from_static_file();

        Ok(Self {
            world,
            assets: Some(Assets::new(&asset_cfg)),
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        println!("{:?}", event);

        match *event {
            Event::Key(Key::Left, ButtonState::Pressed) => {
                user_move_left(&mut self.world);
            }
            Event::Key(Key::Right, ButtonState::Pressed) => {
                user_move_right(&mut self.world);
            }
            Event::Key(Key::Up, ButtonState::Pressed) => {
                user_move_up(&mut self.world);
            }
            Event::Key(Key::Down, ButtonState::Pressed) => {
                user_move_down(&mut self.world);
            }
            Event::Key(Key::Left, ButtonState::Released) => {
                user_clear_x(&mut self.world);
            }
            Event::Key(Key::Right, ButtonState::Released) => {
                user_clear_x(&mut self.world);
            }
            Event::Key(Key::Up, ButtonState::Released) => {
                user_clear_y(&mut self.world);
            }
            Event::Key(Key::Down, ButtonState::Released) => {
                user_clear_y(&mut self.world);
            }
            Event::Key(Key::Z, ButtonState::Pressed) => {
                user_shoot(&mut self.world);
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        MoveObjects.run_now(&mut self.world);
        MaintainLifetime.run_now(&mut self.world);
        BulletCollisions.run_now(&mut self.world);
        EnemyCollisions.run_now(&mut self.world);
        ItemCollisions.run_now(&mut self.world);
        features::update(&mut self.world);
        self.world.maintain();

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.world.fetch_mut::<Context>().update();

        {
            enemies::spawn(&mut self.world);
            items::spawn(&mut self.world);
        }

        {
            if !user_alive(&mut self.world) {
                window.clear(Color::RED)?;
                return Ok(());
            } else {
                window.clear(Color::BLACK)?;
            }
        }

        let mut assets = self.assets.take().unwrap();

        DrawObjects::new(window, &mut assets).run_now(&mut self.world);

        self.assets = Some(assets);

        Ok(())
    }
}
