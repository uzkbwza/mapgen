use mapgen::*;
use mapgen::level;
use rand::prelude::*;
use rand;
// use the sha functions
use sha2::{ Sha256, Digest };
use rand::prelude::*;
use level::Level;
use rooms_corridors::*;
use bsp::*;

#[macro_use]
extern crate arrayref;

const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}

pub fn main() {
    let hash = create_hash("_-__");
    let seed = array_ref!(hash.as_bytes(), 0, 32);
    let mut rng: StdRng = SeedableRng::from_seed(*seed);

    let mut bsp_level = BspLevel::create(WIDTH, HEIGHT, &hash, &mut rng);
    println!("{}", bsp_level);
}