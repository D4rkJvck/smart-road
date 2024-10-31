mod actions;
mod attributes;
mod enums;
mod state;

// use actions::*;
// use attributes::*;
pub use enums::*;
// use state::*;

use super::Intersection;
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
        match self.into_area(area) {
            true => self.slow_down(),
            false => self.speed_up(),
        }
    }
}
