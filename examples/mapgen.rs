use mapgen::*;

pub fn main() {
    let level = Level::new(70, 30);
    print_map(&level.tile_map);
}