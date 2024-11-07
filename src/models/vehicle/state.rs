use super::{Direction, Vehicle};
use crate::{HEIGHT, SAFETY_DISTANCE, WIDTH};
use sdl2::rect::Rect;

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

    pub fn is_too_close_to(&self, other: &Self) -> bool {
        self.direction == other.direction
            && self.route == other.route
            && self.distance_from(other.area.center()) <= SAFETY_DISTANCE
    }

    pub fn is_behind(&self, other: &Self) -> bool {
        if self.direction != other.direction {
            return false;
        }

        match self.direction {
            Direction::North => self.area.y > other.area.y,
            Direction::East => self.area.x < other.area.x,
            Direction::South => self.area.y < other.area.y,
            Direction::West => self.area.x > other.area.x,
        }
    }

    pub fn has_priority_over(&self, other: &Self) -> bool {
        if self.priority < other.priority {
            return true;
        }

        if self.priority == other.priority && !self.is_too_close_to(other) {
            return true;
        }

        false
    }
}
