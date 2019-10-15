use rand::prelude::*;
use vecmap::*;

pub type TileMap = VecMap<TileType>;

pub struct Level {
    width: i32,
    height: i32,
    pub tile_map: TileMap,
}

impl Level {
    pub fn new(width: i32, height: i32) -> Self {
        let tile_map = TileMap::filled_with(TileType::Wall, width, height);
        Level {
            width,
            height,
            tile_map
        }
    }
}

pub fn print_map(map: &TileMap) {
    let mut prev_row = 0;
    for (i, tile) in map.items.iter().enumerate() {
        let (x, y) = map.idx_xy(i);
        if y > prev_row {
            print!("\n");
        }

        let tile_char = match tile {
            TileType::Floor => '.',
            TileType::Wall => '#',
            _ => '?'
        };
        print!("{}", tile_char);
        prev_row = y;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TileType {
    Floor,
    Wall,
}
