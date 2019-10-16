use crate::level::{Level};
use crate::TileType;
use crate::TileMap;
use crate::room::Room;
use rand::prelude::*;
use rand;

pub struct RoomsCorridors {
    pub level: Level
}

impl RoomsCorridors {
    pub fn new(width: i32, height: i32,  hash: &String, rng: &mut StdRng) -> Self {
        let tile_map = TileMap::filled_with(TileType::Wall, width, height);
        let level = Level::new(width, height, hash);

        RoomsCorridors {
            level
        }
    }

    pub fn place_rooms(&mut self, rng: &mut StdRng) {

        // configure room sizes
        let max_rooms = 15;
        let min_room_width = 5;
        let max_room_width = 15;
        let min_room_height = 5;
        let max_room_height = 15;

        for _ in 0..max_rooms {
            // place up to max_rooms.
            let width = rng.gen_range(min_room_width, max_room_width);
            let height = rng.gen_range(min_room_height, max_room_height);

            // set position, making sure the room does not go off the board
            let mut x = rng.gen_range(0, self.level.width - width);
            let mut y = rng.gen_range(0, self.level.height - height);

            let mut collides = false;
            let room = Room::new(x, y, width, height);

            // check all rooms to see if any collide
            for other_room in &self.level.rooms {
                if room.intersects(&other_room) {
                    collides = true;
                    break
                }
            }

            if !collides {
                self.level.create_room(&room);
            }
        }
    }

    pub fn place_corridors(&mut self, rng: &mut StdRng) {
        for i in 0..self.level.rooms.len() - 1 {
            let room = self.level.rooms[i];
            let other_room = self.level.rooms[i + 1];
            self.create_corridor(room, other_room, rng)
        }
    }
    fn create_corridor(&mut self, start_room: Room, end_room: Room, rng: &mut StdRng) {
        let x = start_room.center.x;
        let y = start_room.center.y;
        let x2 = end_room.center.x;
        let y2 = end_room.center.y;
        let horizontal_first = rng.gen_bool(0.5);
        match horizontal_first {
            // horizontal
            true => {
                self.level.create_h_corridor_segment(x, x2, y);
                self.level.create_v_corridor_segment(y, y2, x2);
            },

            // vertical
            false => {
                self.level.create_v_corridor_segment(y, y2, x);
                self.level.create_h_corridor_segment(x, x2, y2);
            }
        }

    }

}