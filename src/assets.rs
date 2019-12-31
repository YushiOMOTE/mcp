use crate::components::*;
use quicksilver::graphics::{Background::Col, Color};
use quicksilver::prelude::*;
use std::collections::HashMap;

pub struct Assets {
    assets: HashMap<AssetId, Image>,
}

fn load_assets() -> HashMap<AssetId, Image> {
    let mut map = HashMap::new();

    let img = Image::load("ship.png").wait().unwrap();
    map.insert(
        AssetId::new(0),
        img.subimage(Rectangle::new((0.0, 0.0), (12.0, 24.0))),
    );
    map.insert(
        AssetId::new(10000),
        img.subimage(Rectangle::new((0.0, 24.0), (12.0, 24.0))),
    );

    let img = Image::load("laser-bolts.png").wait().unwrap();
    map.insert(
        AssetId::new(1),
        img.subimage(Rectangle::new((6.0, 18.0), (5.0, 12.0))),
    );
    map.insert(
        AssetId::new(10001),
        img.subimage(Rectangle::new((20.0, 18.0), (5.0, 12.0))),
    );

    map.insert(
        AssetId::new(3),
        img.subimage(Rectangle::new((6.0, 7.0), (5.0, 5.0))),
    );
    map.insert(
        AssetId::new(10003),
        img.subimage(Rectangle::new((20.0, 7.0), (5.0, 5.0))),
    );

    let img = Image::load("enemy-big.png").wait().unwrap();
    map.insert(
        AssetId::new(2),
        img.subimage(Rectangle::new((3.0, 2.0), (26.0, 30.0))),
    );
    map.insert(
        AssetId::new(10002),
        img.subimage(Rectangle::new((35.0, 2.0), (26.0, 30.0))),
    );

    let img = Image::load("boss.png").wait().unwrap();
    map.insert(AssetId::new(5), img.clone());
    map.insert(AssetId::new(10005), img);

    let img = Image::load("explosion.png").wait().unwrap();
    map.insert(
        AssetId::new(6),
        img.subimage(Rectangle::new((0.0, 0.0), (15.0, 16.0))),
    );
    map.insert(
        AssetId::new(10006),
        img.subimage(Rectangle::new((15.0, 0.0), (15.0, 16.0))),
    );
    map.insert(
        AssetId::new(20006),
        img.subimage(Rectangle::new((32.0, 0.0), (15.0, 16.0))),
    );
    map.insert(
        AssetId::new(30006),
        img.subimage(Rectangle::new((48.0, 0.0), (15.0, 16.0))),
    );

    map
}

impl Assets {
    pub fn new() -> Self {
        Self {
            assets: load_assets(),
        }
    }

    pub fn draw(&self, window: &mut Window, asset: &AssetId, pos: &Pos, counter: u64) {
        let col = match asset.0 {
            0 => Col(Color::RED),
            1 => Col(Color::BLUE),
            2 => Col(Color::GREEN),
            _ => Col(Color::YELLOW),
        };

        let asset = if counter % 20 > 10 {
            AssetId::new(10000 + asset.0)
        } else {
            asset.clone()
        };

        match self.assets.get(&asset) {
            Some(img) => window.draw(&Rectangle::new((pos.x, pos.y), (pos.w, pos.h)), Img(&img)),
            None => window.draw(&Rectangle::new((pos.x, pos.y), (pos.w, pos.h)), col),
        }
    }
}
