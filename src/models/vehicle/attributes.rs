use super::{Direction as dir, Route, Vehicle};
use crate::VEHICLE_SIZE as size;
use sdl2::rect::{Point, Rect};

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

    pub(super) fn distance_from(&self, point: Point) -> i32 {
        let diff_x = self.area.center().x - point.x;
        let diff_y = self.area.center().y - point.y;

        (diff_x - diff_y).abs()
    }

    pub fn sensor_range(&self) -> Rect {
        let range = match self.route {
            Route::Left => size * 3 + 20,
            Route::Straight => size * 4 + 30,
            _ => 0,
        };

        match self.direction {
            dir::North => Rect::new(self.area.x, self.area.y - range, size as u32, range as u32),
            dir::East => Rect::new(self.area.x + size, self.area.y, range as u32, size as u32),
            dir::South => Rect::new(self.area.x, self.area.y + size, size as u32, range as u32),
            dir::West => Rect::new(self.area.x - range, self.area.y, range as u32, size as u32),
        }
    }
}
