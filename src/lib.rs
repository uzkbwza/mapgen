use rand::prelude::*;
pub mod level;
pub mod room;
pub mod rooms_corridors;
pub mod bsp;
use vecmap::*;

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