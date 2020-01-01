use crate::components::*;
use futures::*;
use quicksilver::graphics::{Background::Col, Color};
use quicksilver::prelude::*;
use std::collections::HashMap;

pub struct Assets {
    loading: Box<dyn Future<Item = HashMap<AssetId, Image>, Error = quicksilver::Error>>,
    assets: Option<HashMap<AssetId, Image>>,
}

fn load_assets() -> Box<dyn Future<Item = HashMap<AssetId, Image>, Error = quicksilver::Error>> {
    let mut map = HashMap::new();

    let fut = Image::load("ship.png")
        .map(move |img| {
            map.insert(
                AssetId::new(0),
                img.subimage(Rectangle::new((0.0, 0.0), (12.0, 24.0))),
            );
            map.insert(
                AssetId::new(10000),
                img.subimage(Rectangle::new((0.0, 24.0), (12.0, 24.0))),
            );
            map
        })
        .and_then(|map| Image::load("laser-bolts.png").map(move |img| (map, img)))
        .map(|(mut map, img)| {
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
            map
        })
        .and_then(|map| Image::load("enemy-big.png").map(move |img| (map, img)))
        .map(|(mut map, img)| {
            map.insert(
                AssetId::new(2),
                img.subimage(Rectangle::new((3.0, 2.0), (26.0, 30.0))),
            );
            map.insert(
                AssetId::new(10002),
                img.subimage(Rectangle::new((35.0, 2.0), (26.0, 30.0))),
            );
            map
        })
        .and_then(|map| Image::load("boss.png").map(move |img| (map, img)))
        .map(|(mut map, img)| {
            map.insert(AssetId::new(5), img.clone());
            map.insert(AssetId::new(10005), img);
            map
        })
        .and_then(|map| Image::load("explosion.png").map(move |img| (map, img)))
        .map(|(mut map, img)| {
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
        })
        .and_then(|map| Image::load("background.png").map(move |img| (map, img)))
        .map(|(mut map, img)| {
            map.insert(AssetId::new(7), img.clone());
            map.insert(AssetId::new(10007), img);
            map
        });

    Box::new(fut)
}

impl Assets {
    pub fn new() -> Self {
        Self {
            loading: load_assets(),
            assets: None,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.assets.is_some()
    }

    pub fn poll(&mut self) {
        if self.is_ready() {
            return;
        }

        match self.loading.poll() {
            Ok(Async::Ready(res)) => {
                self.assets = Some(res);
            }
            _ => {}
        }
    }

    pub fn draw(&self, window: &mut Window, animation: &Animation, pos: &Pos, count: u64) {
        let asset = animation.get(count);

        let col = match asset.0 {
            0 => Col(Color::RED),
            1 => Col(Color::BLUE),
            2 => Col(Color::GREEN),
            _ => Col(Color::YELLOW),
        };

        match self.assets.as_ref().unwrap().get(&asset) {
            Some(img) => window.draw_ex(
                &Rectangle::new((pos.x, pos.y), (pos.w, pos.h)),
                Img(&img),
                Transform::IDENTITY,
                pos.z,
            ),
            None => window.draw_ex(
                &Rectangle::new((pos.x, pos.y), (pos.w, pos.h)),
                col,
                Transform::IDENTITY,
                pos.z,
            ),
        }
    }
}
