use rand::prelude::*;
pub mod level;
pub mod room;
pub mod rooms_corridors;
pub mod bsp;
use vecmap::*;
use sha2::*;

// import crates
extern crate sha2;

// arrayref supplies a macro, so add annotation
#[macro_use]
extern crate arrayref;

pub type TileMap = VecMap<TileType>;

#[derive(Copy, Clone, Debug)]
pub enum TileType {
    Floor,
    Wall,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn create_rng(text: &str) -> StdRng {
    let hash = create_hash(text);
    let seed = array_ref!(hash.as_bytes(), 0, 32);
    let rng: StdRng = SeedableRng::from_seed(*seed);
    rng
}
