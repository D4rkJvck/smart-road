mod actions;
mod attributes;
mod enums;
mod state;

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
        x: i32,
        y: i32,
        direction: Direction,
        route: Route,
        shared_sensors: Vec<Point>,
        turn_sensor: Option<Point>,
        distance: i32,
    ) -> Self {
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

    fn ajust_speed(&mut self, collision_area: &Rect, others: Vec<&Vehicle>) {
        if others.iter().any(|other| self.too_close_to(other)) {
            self.speed = 0;
            return;
        }

        if let Some(v) = self.detect_collision(collision_area, others) {
            self.speed = self.distance_from(v.area.center()) / TIME;
            return;
        };

        match self.turn_sensor {
            None => self.speed = self.distance / TIME,
            Some(point) => {
                self.speed = match (self.turned, self.distance_from(point)) {
                    (false, 1..=20) => 1,
                    // (false, 4..=20) => ,
                    _ => self.distance / TIME,
                }
            }
        };
    }

    pub fn detect_collision<'a>(
        &'a self,
        collision_area: &Rect,
        others: Vec<&'a Vehicle>,
    ) -> Option<&Vehicle> {
        if self.route == Route::Right || !self.into_area(collision_area) {
            return None;
        };

        let mut collidable_vehicles = self.collidable_vehicles(&others);
        collidable_vehicles.sort_by_key(|v| self.distance_from(v.area.center()));

        for other in collidable_vehicles {
            if other.into_area(&self.sensor_range()) {
                return Some(other);
            }
        }

        None
    }
}
