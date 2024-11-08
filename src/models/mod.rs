mod vehicle;
mod utils;

use utils::initial_position;
use crate::{GAP, MID_HEIGHT, MID_WIDTH};
use sdl2::rect::{Point, Rect};
pub use vehicle::{Category, Direction, Route, Vehicle};

pub type SensorGrid = [[Point; 6]; 6];

pub struct Road {
    pub intersection: Rect,
    pub sensors: SensorGrid,
    pub vehicles: Vec<Vehicle>,
}

impl Road {
    pub fn new() -> Self {
        let intersection = Rect::new(
            MID_WIDTH - GAP * 3,
            MID_HEIGHT - GAP * 3,
            GAP as u32 * 6 + 1,
            GAP as u32 * 6 + 1,
        );

        let mut sensors: SensorGrid = [[Point::new(0, 0); 6]; 6];

        for x in 0..6 {
            for y in 0..6 {
                sensors[x][y] = Rect::new(
                    intersection.x + GAP * x as i32,
                    intersection.y + GAP * y as i32,
                    GAP as u32,
                    GAP as u32,
                )
                .center()
            }
        }

        Self {
            intersection,
            sensors,
            vehicles: Vec::new(),
        }
    }

    pub fn new_vehicle(&mut self, direction: Direction) {
        let route = Route::random();
        let category = Category::random();
        let (x, y) = initial_position(&direction, &route);

        let new_vehicle = Vehicle::new(x, y, direction, route, category);

        if !new_vehicle.violate_safety_distance(self.vehicles.iter().collect()) {
            self.vehicles.push(new_vehicle)
        };
    }
}
