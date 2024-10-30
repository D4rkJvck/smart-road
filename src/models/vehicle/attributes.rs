use sdl2::rect::Rect;

use super::{Direction, Speed, Vehicle};
use crate::{HEIGHT, SAFETY_DISTANCE, WIDTH};

impl Vehicle {
    /// The velicity method gives
    /// the amount of number of
    /// pixels for the vehicle
    /// to translate.
    pub fn velocity(&self) -> i32 {
        match self.speed {
            Speed::Fast => 3,
            Speed::Normal => 2,
            Speed::Slow => 1,
            Speed::Stop => 0,
        }
    }

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

    /// This function is crucial when
    /// it comes to remove vehicule
    /// as it confirm that the
    /// vehicle is out of the window.
    pub fn is_visible(&self) -> bool {
        self.area.top() < HEIGHT as i32
            && self.area.left() < WIDTH as i32
            && self.area.right() > 0
            && self.area.bottom() > 0
    }

    pub fn into_intersection(&mut self, area: &Rect) -> bool {
        self.area.right() > area.left()
            && self.area.left() < area.right()
            && self.area.top() < area.bottom()
            && self.area.bottom() > area.top()
    }

    pub fn too_close_to(&self, other: &Self) -> bool {
        let diff_x = self.area.center().x - other.area.center().x;
        let diff_y = self.area.center().y - other.area.center().y;

        let distance = diff_x * diff_y;

        self.direction == other.direction && distance <= SAFETY_DISTANCE
    }
}
