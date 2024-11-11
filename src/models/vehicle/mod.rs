mod actions;
mod attributes;
mod enums;
mod state;

use std::time::Instant;

use super::SensorGrid;
use crate::{VEHICLE_HEIGHT, VEHICLE_WIDTH};
pub use enums::*;
use sdl2::rect::Rect;

#[derive(Clone, Copy, PartialEq)]

pub struct Vehicle {
    pub area: Rect,
    speed: Speed,
    direction: Direction,
    route: Route,
    pub category: Category,
    // texture: Texture<'static>,
    pub crossed: bool,
    priority: bool,
    time_interval: (Instant, Instant),
    // distance: ?,
    // velocity: ?,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, direction: Direction, route: Route, category: Category) -> Self {
        Self {
            area: Rect::new(x, y, VEHICLE_WIDTH as u32, VEHICLE_HEIGHT as u32),
            speed: Speed::Fast,
            direction,
            route,
            category,
            crossed: false,
            priority: true,
            time_interval: (Instant::now(), Instant::now()),
        }
    }

    /// This is the function
    /// responsible for the
    /// translation by
    /// updating the position.
    pub fn drive(&mut self, sensors: &SensorGrid, others: Vec<&Vehicle>) {
        self.ajust_speed(sensors, others.clone());
        self.navigate(sensors);
        self.avoid_collision(others, sensors);
        self.movement();
    }

    fn ajust_speed(&mut self, sensors: &SensorGrid, others: Vec<&Vehicle>) {
        if self.violate_safety_distance(others) {
            self.slow_down();
        } else {
            match self.turning_point(sensors) {
                None => {}
                Some(point) => match (self.crossed, self.distance_from(point)) {
                    (false, 1..=50) => self.speed = Speed::Slow,
                    (false, 51..=100) => self.speed = Speed::Normal,
                    _ => self.speed_up(),
                },
            };
        }
    }

    fn avoid_collision(&mut self, others: Vec<&Vehicle>, sensors: &SensorGrid) {
        
    }
}
