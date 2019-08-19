use crate::components::AssetId;
use derive_new::new;
use log::*;
use quicksilver::{geom::Rectangle, graphics::Image, prelude::*};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(new, Serialize, Deserialize)]
struct AssetInfo {
    /// Path to image
    image: String,
    /// Info about how to crop
    crop: Vec<CropInfo>,
}

#[derive(new, Serialize, Deserialize)]
struct CropInfo {
    /// Asset id
    aid: AssetId,
    /// Rectangle to crop
    rect: (f32, f32, f32, f32),
}

#[derive(new)]
struct Crop {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Crop {
    fn crop(&self, x: f32, y: f32, w: f32, h: f32) -> Self {
        assert!(x + w <= self.w);
        assert!(y + h <= self.h);

        Self {
            x: self.x + x,
            y: self.y + y,
            w,
            h,
        }
    }

    fn asset(&self, aid: AssetId) -> CropInfo {
        CropInfo::new(aid, (self.x, self.y, self.w, self.h))
    }
}

pub type AssetsMap = HashMap<AssetId, Image>;

pub fn load_image(s: &str) -> Image {
    info!("Loading {}", s);
    let img = Image::load(s).wait().unwrap();
    info!("Loaded {}", s);
    img
}

///
/// Ground assets
///
/// id = 1xxyy
///
/// where:
///   xx = x-coordinate in the sprite (0-3)
///   yy = y-coordinate in the sprite (0-3)
///
fn process_ground(img: &Image, assets: &mut AssetsMap) {
    let gnd = img.subimage(Rectangle::new((400.0, 400.0), (240.0, 240.0)));
    for y in 0..(240 / 60) {
        for x in 0..(240 / 60) {
            let aid = AssetId(10000 + x * 100 + y);
            let (x, y) = (x as f32, y as f32);
            let simg = gnd.subimage(Rectangle::new((x * 60.0, y * 60.0), (60.0, 60.0)));
            assets.insert(aid, simg);
        }
    }
}

fn crop_ground_assets() -> Vec<CropInfo> {
    let crop = Crop::new(400.0, 400.0, 240.0, 240.0);

    (0..4)
        .map(|y| (0..4).map(move |x| (x, y)))
        .flatten()
        .map(|(x, y)| {
            crop.crop(x as f32 * 60.0, y as f32 * 60.0, 60.0, 60.0)
                .asset(AssetId(10000 + x * 100 + y))
        })
        .collect()
}

///
/// Plate assets
///
/// id = 2xx00
///
/// where:
///   xx = x-coordinate in the sprite (0-3)
///
fn process_plate(img: &Image, assets: &mut AssetsMap) {
    let plt = img.subimage(Rectangle::new((640.0, 240.0), (240.0, 75.0)));

    for x in 0..(240 / 60) {
        let aid = AssetId(20000 + x * 100);
        let x = x as f32;
        let simg = plt.subimage(Rectangle::new((x * 60.0, 0.0), (60.0, 75.0)));
        assets.insert(aid, simg);
    }
}

fn crop_plate_assets() -> Vec<CropInfo> {
    let crop = Crop::new(640.0, 240.0, 240.0, 75.0);

    (0..4)
        .map(|x| {
            crop.crop(x as f32 * 60.0, 0.0, 60.0, 75.0)
                .asset(AssetId(20000 + x * 100))
        })
        .collect()
}

///
/// Tree asset
///
/// id = 30000
///
fn process_tree(img: &Image, assets: &mut AssetsMap) {
    let tree = img.subimage(Rectangle::new((970.0, 405.0), (220.0, 315.0)));
    assets.insert(AssetId(30000), tree);
}

fn crop_tree_assets() -> Vec<CropInfo> {
    vec![Crop::new(970.0, 405.0, 220.0, 315.0).asset(AssetId(30000))]
}

///
/// Grass assets
///
/// id = 400tt
///
/// where:
///   tt = assets type
///
///     0: grass 1
///     1: grass 2
///     2: grass 3
///     3: grass 4
///     4: with flower 1
///     5: with flower 2
///
fn process_grass(img: &Image, assets: &mut AssetsMap) {
    let g = img.subimage(Rectangle::new((720.0, 690.0), (60.0, 30.0)));
    assets.insert(AssetId(40000), g);
    let g = img.subimage(Rectangle::new((880.0, 695.0), (65.0, 25.0)));
    assets.insert(AssetId(40001), g);
    let g = img.subimage(Rectangle::new((880.0, 765.0), (60.0, 35.0)));
    assets.insert(AssetId(40002), g);
    let g = img.subimage(Rectangle::new((885.0, 845.0), (75.0, 35.0)));
    assets.insert(AssetId(40003), g);

    let g = img.subimage(Rectangle::new((720.0, 770.0), (80.0, 30.0)));
    assets.insert(AssetId(40004), g);
    let g = img.subimage(Rectangle::new((720.0, 885.0), (60.0, 25.0)));
    assets.insert(AssetId(40005), g);
}

fn crop_grass_assets() -> Vec<CropInfo> {
    vec![
        Crop::new(720.0, 690.0, 60.0, 30.0).asset(AssetId(40000)),
        Crop::new(880.0, 695.0, 65.0, 25.0).asset(AssetId(40001)),
        Crop::new(880.0, 765.0, 60.0, 35.0).asset(AssetId(40002)),
        Crop::new(885.0, 845.0, 75.0, 35.0).asset(AssetId(40003)),
        Crop::new(720.0, 770.0, 80.0, 30.0).asset(AssetId(40004)),
        Crop::new(720.0, 885.0, 60.0, 25.0).asset(AssetId(40005)),
    ]
}

///
/// Bridge assets
///
/// id = 50uxx
///
/// where:
///
///    u: upper level if 1 (0 or 1)
///    xx: x-cordinate (0-2)
///
fn process_bridge(img: &Image, assets: &mut AssetsMap) {
    let bdg = img.subimage(Rectangle::new((0.0, 680.0), (160.0, 120.0)));

    // Upper 1,2,3
    let b = bdg.subimage(Rectangle::new((55.0, 0.0), (60.0, 40.0)));
    assets.insert(AssetId(50100), b);
    let b = bdg.subimage(Rectangle::new((65.0, 0.0), (60.0, 40.0)));
    assets.insert(AssetId(50101), b);
    let b = bdg.subimage(Rectangle::new((80.0, 0.0), (60.0, 40.0)));
    assets.insert(AssetId(50102), b);

    // Lower
    let b = bdg.subimage(Rectangle::new((80.0, 40.0), (60.0, 20.0)));
    assets.insert(AssetId(50000), b);
}

fn crop_bridge_assets() -> Vec<CropInfo> {
    let crop = Crop::new(0.0, 680.0, 160.0, 120.0);

    vec![
        crop.crop(55.0, 0.0, 60.0, 40.0).asset(AssetId(50100)),
        crop.crop(65.0, 0.0, 60.0, 40.0).asset(AssetId(50101)),
        crop.crop(80.0, 0.0, 60.0, 40.0).asset(AssetId(50102)),
        crop.crop(80.0, 40.0, 60.0, 20.0).asset(AssetId(50000)),
    ]
}

///
/// Ledge assets
///
/// id = 6000d
///
/// where:
///
///   d: left (0) or right (1)
///
fn process_ledge(img: &Image, assets: &mut AssetsMap) {
    let ldg = img.subimage(Rectangle::new((720.0, 480.0), (160.0, 80.0)));

    let b = ldg.subimage(Rectangle::new((20.0, 0.0), (60.0, 60.0)));
    assets.insert(AssetId(60000), b);
    let b = ldg.subimage(Rectangle::new((80.0, 0.0), (60.0, 60.0)));
    assets.insert(AssetId(60001), b);
}

fn crop_ledge_assets() -> Vec<CropInfo> {
    let crop = Crop::new(720.0, 480.0, 160.0, 80.0);

    vec![
        crop.crop(20.0, 0.0, 60.0, 60.0).asset(AssetId(60000)),
        crop.crop(80.0, 0.0, 60.0, 60.0).asset(AssetId(60001)),
    ]
}

fn process_tilesets(assets: &mut AssetsMap) {
    let img = load_image("tileset.png");

    process_ground(&img, assets);
    process_plate(&img, assets);
    process_tree(&img, assets);
    process_grass(&img, assets);
    process_bridge(&img, assets);
    process_ledge(&img, assets);
}

fn gen_assets() -> Vec<AssetInfo> {
    let gnd = AssetInfo::new(
        "tileset.png".into(),
        crop_ground_assets()
            .into_iter()
            .chain(crop_plate_assets())
            .chain(crop_tree_assets())
            .chain(crop_bridge_assets())
            .chain(crop_grass_assets())
            .chain(crop_ledge_assets())
            .collect(),
    );
    vec![gnd]
}

pub fn write_assets(path: &str) {
    let assets = serde_json::to_vec(&gen_assets()).unwrap();

    use std::fs::File;
    use std::io::prelude::*;
    let mut f = File::create(path).unwrap();
    f.write(&assets);
}

fn parse_assets(s: &[u8]) -> Vec<AssetInfo> {
    serde_json::from_slice(s).unwrap()
}

fn import_assets(assets: Vec<AssetInfo>) -> AssetsMap {
    assets
        .into_iter()
        .map(|a| {
            let img = load_image(&a.image);

            a.crop.into_iter().map(move |crop| {
                let r = crop.rect;
                let subimg = img.subimage(Rectangle::new((r.0, r.1), (r.2, r.3)));
                (crop.aid, subimg)
            })
        })
        .flatten()
        .collect()
}

pub fn read_assets(path: &str) -> AssetsMap {
    use std::fs::File;
    use std::io::prelude::*;

    let mut v = Vec::new();
    let mut f = File::open(path).unwrap();
    f.read_to_end(&mut v);

    import_assets(parse_assets(&v))
}

pub fn load_assets() -> AssetsMap {
    let mut assets = HashMap::new();

    // process_tilesets(&mut assets);

    assets.extend(read_assets("assets.json"));

    assets.insert(AssetId(1), load_image("ferris.png"));
    assets.insert(AssetId(2), load_image("ferris-f.png"));

    assets.insert(AssetId(100), load_image("bubble.png"));

    assets.insert(AssetId(901), load_image("skybg.png"));
    assets.insert(AssetId(902), load_image("water.png"));
    assets.insert(AssetId(903), load_image("water-reflex.png"));
    assets.insert(AssetId(904), load_image("clouds.png"));

    assets
}
