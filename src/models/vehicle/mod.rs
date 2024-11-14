mod actions;
mod attributes;
mod enums;
mod state;

use super::SensorGrid;
use crate::{TIME, VEHICLE_HEIGHT, VEHICLE_WIDTH};
pub use enums::*;
use sdl2::rect::Rect;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
pub struct Vehicle {
    pub area: Rect,
    speed: i32,
    direction: Direction,
    pub route: Route,
    pub category: Category,
    // texture: Texture<'static>,
    pub crossed: bool,
    priority: bool,
    time_interval: (Instant, Instant),
    distance: i32,
    // velocity: ?,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, direction: Direction, route: Route, category: Category) -> Self {
        let distance = match route {
            Route::Straight => 550,
            Route::Right => 300,
            Route::Left => 600,
        };

        Self {
            area: Rect::new(x, y, VEHICLE_WIDTH as u32, VEHICLE_HEIGHT as u32),
            speed: distance / TIME,
            direction,
            route,
            category,
            crossed: false,
            priority: true,
            time_interval: (Instant::now(), Instant::now()),
            distance,
        }
    }

    /// This is the function
    /// responsible for the
    /// translation by
    /// updating the position.
    pub fn drive(&mut self, sensors: &SensorGrid, others: Vec<&Vehicle>) {
        self.ajust_speed(sensors, others);
        // if !self.detect_collision(others, sensors) {
        self.navigate(sensors);
        self.movement();
        // } else {
        //     println!("Collision detected, vehicle is stopping.");
        // }
    }

    fn ajust_speed(&mut self, sensors: &SensorGrid, others: Vec<&Vehicle>) {
        if self.violate_safety_distance(&others) {
            self.speed = 0;
            return
        }
        
        if let Some(v) = self.detect_collision(&others, sensors) {
            self.speed = self.distance_from(v.area.center()) / TIME;
            return;
        };

        match self.turning_point(sensors) {
            None => self.speed = self.distance / TIME,
            Some(point) => {
                self.speed = match (self.crossed, self.distance_from(point)) {
                    (false, 1..=20) => 1,
                    // (false, 4..=20) => ,
                    _ => self.distance / TIME,
                }
            }
        };
    }

    pub fn detect_collision<'a>(&'a self, others: &'a Vec<&Vehicle>, sensors: &SensorGrid) -> Option<&Vehicle> {
        if self.route == Route::Right {
            return None
        };

        let mut collidable_vehicles = self.collidable_vehicles(&others, sensors);
        collidable_vehicles.sort_by_key(|v| self.distance_from(v.area.center()));

        for other in collidable_vehicles {
            if other.into_area(&self.sensor_range()) {
                println!("Collision detected with vehicle at area {:?}", other.area.center());
                return Some(other);
            }
        }
        // println!("No collision detected for vehicle at area {:?}", self.area);
        None
    }
}
