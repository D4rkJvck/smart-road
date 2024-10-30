use rand::{thread_rng, Rng};
use sdl2::rect::Rect;
use std::time::Instant;

use crate::{HEIGHT, VEHICLE_HEIGHT, VEHICLE_WIDTH, WIDTH};

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
    texture: String,
    // time_interval: (u64, u64),
    // sensor_range: Rect,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, direction: Direction, route: Route) -> Self {
        Self {
            area: Rect::new(x, y, VEHICLE_WIDTH, VEHICLE_HEIGHT),
            speed: Speed::Fast,
            direction,
            route,
            texture: String::from("./assets/red_car.png"),
            // time_interval: (0, 0),
            // sensor_range: Rect::new(0, 0, 10, 10)
        }
    }

    /// This is the function
    /// responsible for the
    /// translation by
    /// updating the position.
    pub fn drive(&mut self) {
        match (&self.direction, &self.speed, &self.route) {
            (Direction::North, Speed::Fast, Route::Straight) => self.area.y -= 3,
            _ => {}
        };
    }

    pub fn moving(&mut self) {}

    /// This function is crucial when
    /// it comes to remove vehicule
    /// as it confirm that the vehicle is
    /// out of the window...
    pub fn is_visible(&self) -> bool {
        self.area.top() < HEIGHT as i32
            && self.area.left() < WIDTH as i32
            && self.area.right() > 0
            && self.area.bottom() > 0
    }
}
