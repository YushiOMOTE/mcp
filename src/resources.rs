use quicksilver::prelude::*;

pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 600.0;

#[derive(Default, Clone)]
pub struct Context {
    pub count: u64,
    pub gameover: bool,
}

impl Context {
    pub fn new() -> Self {
        Self {
            count: 0,
            gameover: false,
        }
    }

    pub fn update(&mut self) {
        self.count += 1;
    }
}

#[derive(Default, Clone)]
pub struct Events {
    event: Vec<Event>,
}

impl Events {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.event.clear();
    }

    pub fn push(&mut self, event: Event) {
        self.event.push(event);
    }

    pub fn get(&self) -> &[Event] {
        &self.event
    }
}
