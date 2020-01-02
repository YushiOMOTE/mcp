use crate::{assets::*, background::*, components::*, enemies, items, resources::*, systems::*};
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

        enemies::init(&mut world);

        background_spawn(&mut world);

        let e = user_spawn(&mut world);

        world.insert(User::new(e));

        Ok(Self {
            world,
            assets: Some(Assets::new()),
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        if !self.assets.as_ref().unwrap().is_ready() {
            return Ok(());
        }

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
        if !self.assets.as_ref().unwrap().is_ready() {
            self.assets.as_mut().unwrap().poll();
            return Ok(());
        }

        MoveObjects.run_now(&mut self.world);
        MaintainLifetime.run_now(&mut self.world);
        BulletCollisions.run_now(&mut self.world);
        EnemyCollisions.run_now(&mut self.world);
        ItemCollisions.run_now(&mut self.world);
        enemies::update(&mut self.world);
        self.world.maintain();

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.world.fetch_mut::<Context>().update();

        {
            enemies::spawn(&mut self.world);
            items::spawn(&mut self.world);
        }

        if !self.assets.as_ref().unwrap().is_ready() {
            window.clear(Color::BLACK)?;
            return Ok(());
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
