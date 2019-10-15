use mapgen::*;
use mapgen::level;
use rand::prelude::*;
use rand;
// use the sha functions
use sha2::{ Sha256, Digest };
use rand::prelude::*;
use level::Level;

#[macro_use]
extern crate arrayref;

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}

pub fn main() {
    let hash = create_hash("___");
    let seed = array_ref!(hash.as_bytes(), 0, 32);
    let mut rng: StdRng = SeedableRng::from_seed(*seed);

    let mut level = level::Level::new(100, 100);
    level.place_rooms(&mut rng);
    level.place_corridors(&mut rng);
    println!("{}", level);
}