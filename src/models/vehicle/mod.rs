mod actions;
mod attributes;
mod enums;
mod state;

use std::time::Instant;

use super::utils::get_initial_position;
use crate::{controller::Stats, VEHICLE_SIZE};
pub use enums::*;
use sdl2::rect::{Point, Rect};

#[derive(Clone, PartialEq)]
pub struct Vehicle {
    pub area: Rect,
    speed: Speed,
    direction: Direction,
    route: Route,
    pub shared_sensors: Vec<Point>,
    turn_sensor: Option<Point>,
    turned: bool,
    pub distance: f32,
    pub time: Instant,
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
            area: Rect::new(x, y, VEHICLE_SIZE as u32, VEHICLE_SIZE as u32),
            speed: Speed::Fast,
            direction,
            route,
            shared_sensors,
            turn_sensor,
            turned: false,
            distance,
            time: Instant::now(),
        }
    }

    /// This is the function
    /// responsible for the
    /// translation by
    /// updating the position.
    pub fn drive(&mut self, collision_area: &Rect, others: Vec<&Self>, stats: &mut Stats) {
        others.iter().for_each(|other| if self.into_area(&other.area) {
            stats.collisions += 1;
        });
        
        self.ajust_speed(collision_area, others, stats);
        self.navigate();
        self.movement();
    }
}
