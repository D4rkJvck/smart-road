mod actions;
mod attributes;
mod enums;

use actions::*;
use attributes::*;
pub use enums::*;

use crate::{VEHICLE_HEIGHT, VEHICLE_WIDTH};
use sdl2::rect::Rect;

#[derive(Clone, Copy)]
pub struct Vehicle {
    pub area: Rect,
    speed: Speed,
    direction: Direction,
    route: Route,
    pub img_path: &'static str,
    // time: ?,
    // distance: ?,
    // velocity: ?,
    // sensor_range: Rect,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, direction: Direction, route: Route) -> Self {
        Self {
            area: Rect::new(x, y, VEHICLE_WIDTH, VEHICLE_HEIGHT),
            speed: Speed::Fast,
            direction,
            route,
            img_path: "./assets/car_red.png",
            // time: (0, 0),
            // sensor_range: Rect::new(0, 0, 10, 10)
        }
    }

    /// This is the function
    /// responsible for the
    /// translation by
    /// updating the position.
    pub fn drive(&mut self) {
        self.ajust_speed();
        self.navigate();
        self.movement();
    }

    fn ajust_speed(&mut self) {
        match true {
            _ => {}
        }
    }
    fn navigate(&mut self) {}

    fn movement(&mut self) {
        match &self.direction {
            Direction::North => self.area.y -= self.velocity(),
            Direction::South => self.area.y += self.velocity(),
            Direction::East => self.area.x += self.velocity(),
            Direction::West => self.area.x -= self.velocity(),
        };
    }
}
