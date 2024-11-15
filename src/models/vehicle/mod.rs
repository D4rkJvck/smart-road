mod actions;
mod attributes;
mod enums;
mod state;

use super::utils::get_initial_position;
use crate::{TIME, VEHICLE_HEIGHT, VEHICLE_WIDTH};
pub use enums::*;
use sdl2::rect::{Point, Rect};

#[derive(Clone, PartialEq)]
pub struct Vehicle {
    pub area: Rect,
    speed: i32,
    direction: Direction,
    route: Route,
    shared_sensors: Vec<Point>,
    turn_sensor: Option<Point>,
    turned: bool,
    distance: i32,
}

impl Vehicle {
    pub fn new(
        direction: Direction,
        route: Route,
        shared_sensors: Vec<Point>,
        turn_sensor: Option<Point>,
    ) -> Self {
        let (x, y, distance) = get_initial_position(&direction, &route);

        Self {
            area: Rect::new(x, y, VEHICLE_WIDTH as u32, VEHICLE_HEIGHT as u32),
            speed: distance / TIME,
            direction,
            route,
            shared_sensors,
            turn_sensor,
            turned: false,
            distance,
        }
    }

    /// This is the function
    /// responsible for the
    /// translation by
    /// updating the position.
    pub fn drive(&mut self, collision_area: &Rect, others: Vec<&Vehicle>) {
        self.ajust_speed(collision_area, others);
        self.navigate();
        self.movement();
    }
}
