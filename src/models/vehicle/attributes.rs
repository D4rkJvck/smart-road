use super::{Category, Direction as dir, Route, Vehicle};
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
            dir::North => 0.0,
            dir::East => 90.0,
            dir::South => 180.0,
            dir::West => 270.0,
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

    pub fn collidable_sensors(&self, sensors: &SensorGrid) -> Vec<Point> {
        match (self.direction, self.route) {
            (dir::North, Route::Straight) => vec![sensors[4][4], sensors[4][2], sensors[4][1]],
            (dir::North, Route::Left) => vec![
                sensors[3][4],
                sensors[3][3],
                sensors[3][2],
                sensors[2][2],
                sensors[1][2],
            ],
            (dir::East, Route::Straight) => vec![sensors[1][4], sensors[3][4], sensors[4][4]],
            (dir::East, Route::Left) => vec![
                sensors[1][3],
                sensors[2][3],
                sensors[3][3],
                sensors[3][2],
                sensors[3][1],
            ],
            (dir::South, Route::Straight) => vec![sensors[1][1], sensors[1][3], sensors[1][4]],
            (dir::South, Route::Left) => vec![
                sensors[2][1],
                sensors[2][2],
                sensors[2][3],
                sensors[3][3],
                sensors[3][4],
            ],
            (dir::West, Route::Straight) => vec![sensors[4][1], sensors[2][1], sensors[1][1]],
            (dir::West, Route::Left) => vec![
                sensors[4][2],
                sensors[3][2],
                sensors[2][2],
                sensors[2][3],
                sensors[2][4],
            ],
            _ => vec![],
        }
    }

    pub fn turning_point(&self, sensors: &SensorGrid) -> Option<Point> {
        match (self.direction, self.route) {
            (dir::North, Route::Right) => Some(sensors[5][5]),
            (dir::North, Route::Left) => Some(sensors[3][2]),
            (dir::East, Route::Right) => Some(sensors[0][5]),
            (dir::East, Route::Left) => Some(sensors[3][3]),
            (dir::South, Route::Right) => Some(sensors[0][0]),
            (dir::South, Route::Left) => Some(sensors[2][3]),
            (dir::West, Route::Right) => Some(sensors[5][0]),
            (dir::West, Route::Left) => Some(sensors[2][2]),
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
