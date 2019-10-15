use vecmap::*;
use std::fmt;
use rand;
use rand::prelude::*;
use crate::room::Room;
use crate::TileType;
pub type TileMap = VecMap<TileType>;

#[derive(Debug)]
pub struct Level {
    pub tile_map: TileMap,
    width: i32,
    height: i32,
    rooms: Vec<Room>,
}

impl Level {
    pub fn new(width: i32, height: i32) -> Self {
        let tile_map = TileMap::filled_with(TileType::Wall, width, height);
        Level {
            tile_map,
            width,
            height,
            rooms: Vec::new(),
        }
    }

    pub fn place_rooms(&mut self, rng: &mut StdRng) {

        // configure room sizes
        let max_rooms = 150000;
        let min_room_width = 1;
        let max_room_width = 50;
        let min_room_height = 1;
        let max_room_height = 50;

        for _ in 0..max_rooms {
            // place up to max_rooms.
            let width = rng.gen_range(min_room_width, max_room_width);
            let height = rng.gen_range(min_room_height, max_room_height);

            // set position, making sure the room does not go off the board
            let mut x = rng.gen_range(0, self.width - width);
            let mut y = rng.gen_range(0, self.height - height);

            let mut collides = false;
            let room = Room::new(x, y, width, height);

            // check all rooms to see if any collide
            for other_room in &self.rooms {
                if room.intersects(&other_room) {
                    collides = true;
                    break
                }
            }

            if !collides {
                self.create_room(&room);
            }
        }
    }

    pub fn place_corridors(&mut self, rng: &mut StdRng) {
        for i in 0..self.rooms.len() - 1 {
            let room = self.rooms[i];
            let other_room = self.rooms[i + 1];
            self.create_corridor(room, other_room, rng)
        }
    }
    fn create_corridor(&mut self, start_room: Room, end_room: Room, rng: &mut StdRng) {
        let horizontal_first = rng.gen_bool(0.5);
        let x = start_room.center.x;
        let y = start_room.center.y;
        let x2 = end_room.center.x;
        let y2 = end_room.center.y;
        match horizontal_first {
            // horizontal
            true => {
                self.create_h_corridor_segment(x, x2, y);
                self.create_v_corridor_segment(y, y2, x2);
            },

            // vertical
            false => {
                self.create_v_corridor_segment(y, y2, x);
                self.create_h_corridor_segment(x, x2, y2);
            }
        }

    }

    fn create_h_corridor_segment(&mut self, mut start_x: i32, mut end_x: i32, y: i32) {
        if start_x > end_x {
            std::mem::swap(&mut start_x, &mut end_x)
        }

        for x in start_x..end_x + 1 {
            self.tile_map.set_point(x, y, TileType::Floor);
        }
    }

    fn create_v_corridor_segment(&mut self, mut start_y: i32, mut end_y: i32, x: i32) {
        if start_y > end_y {
            std::mem::swap(&mut start_y, &mut end_y)
        }

        for y in start_y..end_y + 1 {
            self.tile_map.set_point(x, y, TileType::Floor);
        }
    }

    fn create_room(&mut self, room: &Room) {
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