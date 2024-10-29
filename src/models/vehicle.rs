use rand::{thread_rng, Rng};
use sdl2::rect::Rect;

use crate::{VEHICLE_HEIGHT, VEHICLE_WIDTH};

enum Speed {
    Stop,
    Slow,
    Medium,
    Fast,
}

//________________________________________________________________
//

pub enum Route {
    Right,
    Left,
    Straight,
}

impl Route {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..3) {
            0 => Self::Left,
            1 => Self::Straight,
            _ => Self::Right,
        }
    }
}

//________________________________________________________________
//

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..4) {
            0 => Self::North,
            1 => Self::South,
            2 => Self::East,
            _ => Self::West,
        }
    }
}

//________________________________________________________________
//

pub struct Vehicle {
    pub area: Rect,
    speed: Speed,
    direction: Direction,
    route: Route,
    // sensor_range: Rect,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, direction: Direction, route: Route) -> Self {
        Self {
            area: Rect::new(x, y, VEHICLE_WIDTH, VEHICLE_HEIGHT),
            speed: Speed::Fast,
            direction,
            route,
            // sensor_range,
        }
    }

    pub fn drive(&mut self) {
        match (&self.direction, &self.speed, &self.route) {
            (Direction::North, Speed::Fast, Route::Straight) => self.area.y -= 3,
            _ => {}
        };
    }
}
