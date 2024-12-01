mod actions;
mod attributes;
mod enums;
mod state;

use std::time::Instant;

use super::utils::get_initial_position;
use crate::{controller::Stats, VEHICLE_SIZE as size};
pub use enums::*;
use sdl2::rect::{Point, Rect};

#[derive(Clone, PartialEq)]
pub struct Vehicle {
    pub area: Rect,
    pub speed: Speed,
    direction: Direction,
    route: Route,
    pub shared_sensors: Vec<Point>,
    turn_sensor: Option<Point>,
    turned: bool,
    pub distance: f32,
    pub time: Instant,
    on_close_call: bool,
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
            area: Rect::new(x, y, size as u32, size as u32),
            speed: Speed::Fast,
            direction,
            route,
            shared_sensors,
            turn_sensor,
            turned: false,
            distance,
            time: Instant::now(),
            on_close_call: false,
        }
    }

    /// This is the function
    /// responsible for the
    /// translation by
    /// updating the position.
    pub fn drive(&mut self, collision_area: &Rect, others: Vec<&Self>, stats: &mut Stats) {
        self.ajust_speed(collision_area, others, stats);
        self.navigate();
        self.movement();
    }
}
