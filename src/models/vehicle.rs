use rand::{thread_rng, Rng};
use sdl2::rect::Rect;

use crate::{ VEHICLE_HEIGHT, VEHICLE_WIDTH };

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
    area: Rect,
    speed: Speed,
    direction: Direction,
    route: Route,
    // sensor_range: Rect,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, direction: Direction, route: Route) -> Self {
        // use Direction::*;
        // use Route::*;

        // let area = match (&direction, &route) {
        //     (North, Right) => Rect::new((MID_WIDTH + GAP * 2 + 5) as i32, HEIGHT as i32 - 45, 30, 40),
        //     (North, Straight) => Rect::new((MID_WIDTH + GAP + 5) as i32, HEIGHT as i32 - 45, 30, 40),
        //     (North, Left) => Rect::new((MID_WIDTH + 5) as i32, HEIGHT as i32 - 45, 30, 40),
        //     _ => Rect::new(0, 0, 0, 0),
        // };

        Self {
            area: Rect::new(x, y, VEHICLE_WIDTH, VEHICLE_HEIGHT),
            speed: Speed::Fast,
            direction,
            route,
            // sensor_range,
        }
    }
}
