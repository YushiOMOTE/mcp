use crate::{assets::*, components::*, enemies, resources::*, systems::*};
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
        world.register::<Vel>();
        world.register::<Pos>();
        world.register::<Bound>();
        world.register::<Lifetime>();
        world.register::<Animation>();
        world.register::<AssetId>();
        world.register::<Player>();
        world.register::<Enemy>();
        world.register::<Bullet>();
        world.register::<enemies::Normal>();
        world.register::<enemies::Boss>();

        world
            .create_entity()
            .with(Pos::new(0.0, -HEIGHT * 2.0, -10.0, WIDTH, HEIGHT * 2.0))
            .with(Vel::new(0.0, 0.4))
            .with(AssetId::new(7))
            .with(Lifetime::Scroll(HEIGHT * 2.0))
            .build();
        world
            .create_entity()
            .with(Pos::new(0.0, 0.0, -10.0, WIDTH, HEIGHT * 2.0))
            .with(Vel::new(0.0, 0.4))
            .with(AssetId::new(7))
            .with(Lifetime::Scroll(HEIGHT * 2.0))
            .build();

        let e = world
            .create_entity()
            .with(Pos::new(WIDTH / 2.0, HEIGHT * 0.9, 0.0, 15.0, 30.0))
            .with(Player::new(100))
            .with(Vel::new(0.0, 0.0))
            .with(Bound::new(0.0, 0.0, WIDTH, HEIGHT))
            .with(Animation::new(AssetId::new(0), 10).add(AssetId::new(10000), 10))
            .build();

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
        enemies::MoveBoss.run_now(&mut self.world);
        enemies::MoveNormal.run_now(&mut self.world);
        self.world.maintain();

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.world.fetch_mut::<Context>().update();

        {
            let count = self.world.fetch::<Context>().count;

            let num = if count < 1000 {
                40
            } else if count < 1500 {
                100
            } else {
                1000
            };

            if count % num == 0 {
                enemies::spawn_normal(&mut self.world);
            }

            if count == 1000 {
                enemies::spawn_boss(&mut self.world);
            }
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
