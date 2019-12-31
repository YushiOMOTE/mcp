use crate::{assets::*, components::*, effect::*, enemies, resources::*, systems::*};
use quicksilver::{
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color},
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};
use specs::prelude::*;

pub struct Play {
    world: World,
    counter: usize,
    assets: Option<Assets>,
}

impl Play {
    fn next(&mut self) {
        self.counter += 1;

        let num = if self.counter < 1000 {
            40
        } else if self.counter < 1500 {
            100
        } else {
            1000
        };

        if self.counter % num == 0 {
            enemies::spawn_normal(&mut self.world);
        }

        if self.counter == 1000 {
            enemies::spawn_boss(&mut self.world);
        }
    }
}

impl State for Play {
    fn new() -> Result<Self> {
        let mut world = World::new();

        world.insert(Player::new());
        world.insert(Counter::new());
        world.register::<Vel>();
        world.register::<Pos>();
        world.register::<AssetId>();
        world.register::<Enemy>();
        world.register::<Bullet>();
        world.register::<Bomb>();
        world.register::<enemies::Normal>();
        world.register::<enemies::Boss>();

        Ok(Self {
            world,
            counter: 0,
            assets: Some(Assets::new()),
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match *event {
            Event::Key(Key::Left, ButtonState::Pressed) => {
                self.world.fetch_mut::<Player>().move_left();
            }
            Event::Key(Key::Right, ButtonState::Pressed) => {
                self.world.fetch_mut::<Player>().move_right();
            }
            Event::Key(Key::Up, ButtonState::Pressed) => {
                self.world.fetch_mut::<Player>().move_up();
            }
            Event::Key(Key::Down, ButtonState::Pressed) => {
                self.world.fetch_mut::<Player>().move_down();
            }
            Event::Key(Key::Z, ButtonState::Pressed) => {
                let p = (*self.world.fetch::<Player>()).clone();
                p.shoot(&mut self.world);
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.next();
        self.world.fetch_mut::<Counter>().increment();
        MoveObjects.run_now(&mut self.world);
        CollectGarbages.run_now(&mut self.world);
        BulletCollisions.run_now(&mut self.world);
        EnemyCollisions.run_now(&mut self.world);
        enemies::move_enemies(&mut self.world);
        self.world.maintain();

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        {
            let player = self.world.fetch::<Player>();
            if player.life == 0 {
                window.clear(Color::RED)?;
                return Ok(());
            } else {
                window.clear(Color::BLACK)?;
            }
        }

        let mut assets = self.assets.take().unwrap();

        {
            let player = self.world.fetch::<Player>();
            let counter = self.world.fetch::<Counter>();

            assets.draw(window, &player.asset, &player.pos, counter.count);
        }

        DrawObjects::new(window, &mut assets).run_now(&mut self.world);
        ActionBomb::new(window, &mut assets).run_now(&mut self.world);

        self.assets = Some(assets);

        Ok(())
    }
}
