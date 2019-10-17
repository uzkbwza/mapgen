use crate::room::Room;
use rand::prelude::*;
use rand;
use crate::level::Level;

pub struct Leaf {
    min_size: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    left_child: Option<Box<Leaf>>,
    right_child: Option<Box<Leaf>>,
    room: Option<Room>,
}

impl Leaf {
    pub fn new(x: i32, y: i32, width: i32, height: i32, min_size: i32) -> Self {
        Leaf {
            min_size,
            x,
            y,
            width,
            height,
            left_child: None,
            right_child: None,
            room: None
        }
    }

    fn get_room(&self) -> Option<Room> {
        if self.is_leaf() {
            return self.room
        }

        let mut left_room = None;
        let mut right_room = None;

        if let Some(ref leaf) = self.left_child {
            left_room = leaf.get_room();
        };

        if let Some(ref leaf) = self.right_child {
            right_room = leaf.get_room();
        };

        match (left_room, right_room) {
            (None, None) => None,
            (Some(room), _) => Some(room),
            (_, Some(room)) => Some(room),
        }
    }

    fn create_rooms(&mut self, rng: &mut StdRng, rooms: &mut Vec<Room>) {
        if let Some(ref mut leaf) = self.left_child {
            leaf.as_mut().create_rooms(rng, rooms);
        };

        if let Some(ref mut leaf) = self.right_child {
            leaf.as_mut().create_rooms(rng, rooms);
        };

        let min_room_width = 3;
        let min_room_height = 5;

        if self.is_leaf() {
            let width = rng.gen_range(min_room_width, self.width);
            let height = rng.gen_range(min_room_height, self.height);
            let x = rng.gen_range(0, self.width - width);
            let y = rng.gen_range(0, self.height - height);
            self.room = Some(Room::new(x + self.x, y + self.y, width, height));
            rooms.push(self.room.unwrap());
        }

        if let (Some(ref mut left), Some(ref mut right)) = (&mut self.left_child, &mut self.right_child) {
            Self::create_corridors(rng, left, right, rooms);
        }
    }

    fn create_corridors(rng: &mut StdRng, left: &mut Box<Leaf>, right: &mut Box<Leaf>, corridors: &mut Vec<Room>) {
        if let (Some(left_room), Some(right_room)) = (left.get_room(), right.get_room()) {

            let (x, y) = (
                rng.gen_range(left_room.x, left_room.x + left_room.width),
                rng.gen_range(left_room.y, left_room.y + left_room.height),
            );

            let (x2, y2) = (
                rng.gen_range(right_room.x, right_room.x + right_room.width),
                rng.gen_range(right_room.y, right_room.y + right_room.height),
            );

            let horizontal_first = rng.gen_bool(0.5);
            match horizontal_first {
                // horizontal
                true => {
                    corridors.push(Self::create_h_corridor_segment(x, x2, y));
                    corridors.push(Self::create_v_corridor_segment(y, y2, x2));
                },

                // vertical
                false => {
                    corridors.push(Self::create_v_corridor_segment(y, y2, x));
                    corridors.push(Self::create_h_corridor_segment(x, x2, y2));
                }
            };
        }
    }

    pub fn create_h_corridor_segment(mut start_x: i32, mut end_x: i32, y: i32) -> Room {
        if start_x > end_x {
            std::mem::swap(&mut start_x, &mut end_x)
        }

        Room::new(start_x, y, (end_x - start_x) + 1, 1)
    }

    pub fn create_v_corridor_segment(mut start_y: i32, mut end_y: i32, x: i32) -> Room {
        if start_y > end_y {
            std::mem::swap(&mut start_y, &mut end_y)
        }
        Room::new(x, start_y, 1, end_y - start_y)
    }

    fn is_leaf(&self) -> bool {
        match self.left_child {
            None => match self.right_child {
                None => true,
                _ => false
            },
            _ => false
        }
    }

    fn generate(&mut self, rng: &mut StdRng) {
        if self.is_leaf() {
            if self.split(rng) {
                self.left_child.as_mut().unwrap().generate(rng);
                self.right_child.as_mut().unwrap().generate(rng);
            }
        }
    }

    fn split(&mut self, rng: &mut StdRng) -> bool {
        let mut split_horizontal = rng.gen_bool(0.5);

        // if width is > 125% of height,
        if self.width > self.height && (self.width as f32 / self.height as f32) >= 1.25 {

            // split vertically
            split_horizontal = false

        // do opposite for height
        } else if self.height > self.width && (self.height as f32 / self.width as f32) >= 1.25 {
            split_horizontal = true
        }

        let max_split = match split_horizontal {
            true => self.height - self.min_size,
            false => self.width - self.min_size,
        };

        // as the tree splits, the area shrinks. if the maximum split size is still smaller
        // than the minimum leaf size, stop splitting.
        if max_split <= self.min_size {
            return false;
        }

        let split_position = rng.gen_range(self.min_size, max_split);

        if split_horizontal {
            self.left_child = Some(Box::new(Leaf::new(
                self.x,
                self.y,
                self.width,
                split_position,
                self.min_size
            )));

            self.right_child = Some(Box::new(Leaf::new(
                self.x,
                self.y + split_position,
                self.width,
                self.height - split_position,
                self.min_size
            )));

        } else {
            self.left_child = Some(Box::new(Leaf::new(
                self.x,
                self.y,
                split_position,
                self.height,
                self.min_size
            )));

            self.right_child = Some(Box::new(Leaf::new(
                self.x + split_position,
                self.y,
                self.width - split_position,
                self.height,
                self.min_size
            )))
        }
        true
    }
}

pub struct BspLevel {
    level: Level,
}

impl BspLevel {
    pub fn create(width: i32, height: i32, hash: &String) -> Level {
        let rng = crate::create_rng(hash)
        let level = Level::new(width, height, hash);
        let mut map = BspLevel {
            level
        };

        map.place_rooms(rng);

        map.level
    }

    fn place_rooms(&mut self, rng: &mut StdRng) {
        let mut root = Leaf::new(0, 0, self.level.width, self.level.height, 10);
        root.generate(rng);

        let mut rooms = Vec::new();
        root.create_rooms(rng, &mut rooms);
        for room in rooms {
            self.level.create_room(&room);
        }
    }
}