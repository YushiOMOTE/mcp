use crate::components::*;
use futures::*;
use quicksilver::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

type Loading = Box<dyn Future<Item = HashMap<AssetId, Image>, Error = String>>;

#[derive(Deserialize, Clone, Debug)]
pub struct AssetConfig {
    name: String,
    crop: Option<(f32, f32, f32, f32)>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AssetsConfig(HashMap<String, Vec<AssetConfig>>);

impl AssetsConfig {
    pub fn from_static_file() -> Self {
        serde_yaml::from_str(include_str!("config/assets.yml")).expect("Couldn't parse assets file")
    }
}

pub struct Assets {
    assets: HashMap<AssetId, Image>,
    loader: Vec<Loading>,
}

fn load_assets(cfg: &AssetsConfig) -> Vec<Loading> {
    cfg.0
        .clone()
        .into_iter()
        .map(|(name, assets)| {
            let img = Image::load(name.clone());
            let ename = name.clone();

            let fut = img
                .map(move |img| {
                    let mut map = HashMap::new();

                    for asset in assets {
                        let img = match asset.crop {
                            Some(crop) => {
                                img.subimage(Rectangle::new((crop.0, crop.1), (crop.2, crop.3)))
                            }
                            None => img.clone(),
                        };
                        map.insert(AssetId::new(asset.name.clone()), img);
                    }

                    map
                })
                .map_err(move |e| format!("Couldn't load asset: {}: {}", ename, e));

            Box::new(fut) as Loading
        })
        .collect()
}

impl Assets {
    pub fn new(cfg: &AssetsConfig) -> Self {
        Self {
            loader: load_assets(cfg),
            assets: HashMap::new(),
        }
    }

    fn poll(&mut self) {
        while !self.loader.is_empty() {
            match self.loader.last_mut().poll() {
                Ok(Async::Ready(map)) => {
                    for m in map {
                        self.assets.extend(m);
                    }
                }
                Ok(Async::NotReady) => return,
                Err(e) => panic!("Poll error: {}", e),
            }
            self.loader.pop();
        }
    }

    pub fn draw(&mut self, window: &mut Window, animation: &Animation, pos: &Pos, count: u64) {
        self.poll();

        let asset = animation.get(count);
        let asset = match self.assets.get(&asset) {
            Some(asset) => asset,
            None => return,
        };

        window.draw_ex(
            &Rectangle::new((pos.x, pos.y), (pos.w, pos.h)),
            Img(asset),
            Transform::IDENTITY,
            pos.z,
        );
    }
}
