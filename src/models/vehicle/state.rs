use super::{Direction as dir, Route, Vehicle};
use crate::{HEIGHT, SAFETY_DISTANCE, WIDTH};
use sdl2::rect::{Point, Rect};

impl Vehicle {
    /// This function is crucial when
    /// it comes to remove vehicule
    /// as it confirm that the
    /// vehicle is out of the window.
    pub fn is_visible(&self) -> bool {
        self.area.top() <= HEIGHT as i32
            && self.area.left() <= WIDTH as i32
            && self.area.right() >= 0
            && self.area.bottom() >= 0
    }

    pub fn into_area(&self, area: &Rect) -> bool {
        self.area.right() > area.left()
            && self.area.left() < area.right()
            && self.area.top() < area.bottom()
            && self.area.bottom() > area.top()
    }

    pub(in crate::models) fn too_close_to(&self, other: &Self) -> bool {
        self.is_behind(other) && self.distance_from(other.area.center()) < SAFETY_DISTANCE
    }

    pub(super) fn is_behind(&self, other: &Self) -> bool {
        self.direction == other.direction
            && self.route == other.route
            && match self.direction {
                dir::North => self.area.y > other.area.y,
                dir::East => self.area.x < other.area.x,
                dir::South => self.area.y < other.area.y,
                dir::West => self.area.x > other.area.x,
            }
    }

    pub(super) fn has_priority_over(&self, other: &Self) -> bool {
        match self.direction {
            dir::North => other.direction == dir::West,
            dir::East => other.direction == dir::North,
            dir::South => other.direction == dir::East,
            dir::West => other.direction == dir::South,
        }
    }

    pub(super) fn passed_sensor(&self, sensor: Point) -> bool {
        let center = self.area.center();

        match (self.direction, self.route) {
            (dir::North, Route::Straight) => center.y < sensor.y,
            (dir::North, Route::Left) => center.y < sensor.y || center.x < sensor.x,
            (dir::East, Route::Straight) => center.x > sensor.x,
            (dir::East, Route::Left) => center.x > sensor.x || center.y < sensor.y,
            (dir::South, Route::Straight) => center.y > sensor.y,
            (dir::South, Route::Left) => center.y > sensor.y || center.x > sensor.x,
            (dir::West, Route::Straight) => center.x < sensor.x,
            (dir::West, Route::Left) => center.x < sensor.x || center.y > sensor.y,
            _ => false,
        }
    }
}
