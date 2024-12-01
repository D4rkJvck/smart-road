use super::{Direction as dir, Vehicle};
use crate::{GAP, MID_SIZE, VEHICLE_SIZE};
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
        let range = GAP as u32 * 4;
        let shift = MID_SIZE - GAP * 2;
        let size = VEHICLE_SIZE as u32;

        match self.direction {
            dir::North => Rect::new(self.area.x, shift, size, range),
            dir::East => Rect::new(shift, self.area.y, range, size),
            dir::South => Rect::new(self.area.x, shift, size, range),
            dir::West => Rect::new(shift, self.area.y, range, size),
        }
    }
}
