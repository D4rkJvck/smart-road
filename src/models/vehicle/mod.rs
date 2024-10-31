mod actions;
mod attributes;
mod enums;

use actions::*;
use attributes::*;
pub use enums::*;

use crate::{VEHICLE_HEIGHT, VEHICLE_WIDTH};
use sdl2::{rect::Rect, sys::False};

use super::Intersection;

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
            area: Rect::new(x, y, VEHICLE_WIDTH as u32, VEHICLE_HEIGHT as u32),
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
    pub fn drive(&mut self, intersection: &Intersection) {
        self.ajust_speed(&intersection.area);
        self.navigate();
        self.movement();
    }

    fn ajust_speed(&mut self, area: &Rect) {
        self.speed = match self.into_intersection(area) {
            true => Speed::Normal,
            false => Speed::Fast,
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
