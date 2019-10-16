use vecmap::*;
use std::fmt;
use rand;
use rand::prelude::*;
use crate::room::Room;
use crate::{TileType, TileMap};

#[derive(Debug)]
pub struct Level {
    pub tile_map: TileMap,
    pub width: i32,
    pub height: i32,
    pub rooms: Vec<Room>,
    hash: String,
}

impl Level {
    pub fn new(width: i32, height: i32, hash: &String) -> Self {
        let tile_map = TileMap::filled_with(TileType::Wall, width, height);

        Level {
            tile_map,
            width,
            height,
            rooms: Vec::new(),
            hash: hash.clone(),
        }
    }


    pub fn create_h_corridor_segment(&mut self, mut start_x: i32, mut end_x: i32, y: i32) {
        if start_x > end_x {
            std::mem::swap(&mut start_x, &mut end_x)
        }

        for x in start_x..end_x + 1 {
            self.tile_map.set_point(x, y, TileType::Floor);
        }
    }

    pub fn create_v_corridor_segment(&mut self, mut start_y: i32, mut end_y: i32, x: i32) {
        if start_y > end_y {
            std::mem::swap(&mut start_y, &mut end_y)
        }

        for y in start_y..end_y + 1 {
            self.tile_map.set_point(x, y, TileType::Floor);
        }
    }

    pub fn create_room(&mut self, room: &Room) {
        let room = room;
        for x in room.x..room.x2 {
            for y in room.y..room.y2 {
                if let Ok(_) = self.tile_map.retrieve(x,y) {
                    self.tile_map.set_point(x,y, TileType::Floor);
                }
            }
        }
        self.rooms.push(*room);
    }
}


impl fmt::Display for Level {
    fn fmt (&self, f: &mut fmt::Formatter ) -> fmt::Result {
        let mut prev_row = 0;
        for (i, tile) in self.tile_map.items.iter().enumerate() {
            let (_, y) = self.tile_map.idx_xy(i);
            if y > prev_row {
                write!(f, "\n")?;
            }

            let tile_char = match tile {
                TileType::Floor => '.',
                TileType::Wall => '#',
                _ => '?'
            };
            write!(f, "{}", tile_char)?;
            prev_row = y;
        }
        Ok(())
    }
}