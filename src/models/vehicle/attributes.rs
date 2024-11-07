use super::{Category, Direction, Route, Vehicle};
use crate::models::SensorGrid;
use sdl2::rect::Point;
use std::time::Duration;

impl Vehicle {
    /// This method returns the
    /// rotation angle of the
    /// vehicle's picture
    /// depending on the direction.
    pub fn angle(&self) -> f64 {
        match self.direction {
            Direction::North => 0.0,
            Direction::East => 90.0,
            Direction::South => 180.0,
            Direction::West => 270.0,
        }
    }

    pub fn category(&self) -> &'static str {
        match self.category {
            Category::Police => "./assets/cars/police.png",
            Category::Taxi => "./assets/cars/taxi.png",
            Category::Red => "./assets/cars/red.png",
            Category::Black => "./assets/cars/black.png",
        }
    }

    pub fn turning_point(&self, sensors: &SensorGrid) -> Option<Point> {
        match (self.direction, self.route) {
            (Direction::North, Route::Right) => Some(sensors[5][5]),
            (Direction::North, Route::Left) => Some(sensors[3][2]),
            (Direction::East, Route::Right) => Some(sensors[0][5]),
            (Direction::East, Route::Left) => Some(sensors[3][3]),
            (Direction::South, Route::Right) => Some(sensors[0][0]),
            (Direction::South, Route::Left) => Some(sensors[2][3]),
            (Direction::West, Route::Right) => Some(sensors[5][0]),
            (Direction::West, Route::Left) => Some(sensors[2][2]),
            _ => None,
        }
    }

    pub fn distance_from(&self, point: Point) -> i32 {
        let diff_x = self.area.center().x - point.x;
        let diff_y = self.area.center().y - point.y;

        (diff_x - diff_y).abs()
    }

    pub fn time(&self) -> Duration {
        self.time_interval.1 - self.time_interval.0
    }
}
