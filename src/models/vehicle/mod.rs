mod actions;
mod attributes;
mod enums;
mod state;

use super::SensorGrid;
use crate::{SAFETY_DISTANCE, VEHICLE_HEIGHT, VEHICLE_WIDTH};
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
    // time: ?,
    // distance: ?,
    // velocity: ?,
    // sensor_range: Rect,
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
            // time: (0, 0),
            // sensor_range: Rect::new(0, 0, 10, 10)
        }
    }

    /// This is the function
    /// responsible for the
    /// translation by
    /// updating the position.
    pub fn drive(&mut self, intersection: &Rect, sensors: &SensorGrid, others: Vec<&Vehicle>) {
        self.ajust_speed(intersection, others);
        self.navigate(sensors);
        self.movement();
    }

    fn ajust_speed(&mut self, intersection: &Rect, others: Vec<&Vehicle>) {
        let too_close = others.iter().any(|other| {
            self.distance_from(other.area.center()) < SAFETY_DISTANCE && self.is_behind(other)
        });

        if too_close {
            self.slow_down();
        } else {
            self.speed_up();
        }

        match (self.into_area(intersection), self.crossed) {
            (true, false) => self.slow_down(),
            (true, true) => self.speed_up(),
            (false, true) => self.speed_up(),
            _ => {}
        };
    }
}
