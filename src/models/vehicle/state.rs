use super::{Direction as dir, Vehicle};
use crate::{SAFETY_DISTANCE, SIZE};
use sdl2::rect::Rect;

impl Vehicle {
    /// This function is crucial when
    /// it comes to remove vehicule
    /// as it confirm that the
    /// vehicle is out of the window.
    pub fn is_visible(&self) -> bool {
        self.area.top() <= SIZE as i32
            && self.area.left() <= SIZE as i32
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
        self.direction == other.direction
            && self.route == other.route
            && self.is_behind(other)
            && self.distance_from(other.area.center()) < SAFETY_DISTANCE
    }

    pub(super) fn is_behind(&self, other: &Self) -> bool {
        match self.direction {
            dir::North => self.area.y > other.area.y,
            dir::East => self.area.x < other.area.x,
            dir::South => self.area.y < other.area.y,
            dir::West => self.area.x > other.area.x,
        }
    }

    pub(super) fn detect_vehicle(&self, collision_area: &Rect, other: &Self) -> bool {
        self.into_area(collision_area)
            && other.into_area(&self.sensor_range())
            && (self.direction != other.direction || self.route != other.route)
            && self.is_behind(other)
    }

    pub(super) fn has_priority_over(&self, other: &Self) -> bool {
        match self.direction {
            dir::North => other.direction == dir::West,
            dir::East => other.direction == dir::North,
            dir::South => other.direction == dir::East,
            dir::West => other.direction == dir::South,
        }
    }

    pub fn collides(&self, area: &Rect, margin: i32) -> bool {
        let expanded_area = Rect::new(
            self.area.x() - margin,
            self.area.y() - margin,
            ((self.area.width() as i32 + 2 * margin).max(0)) as u32,
            ((self.area.height() as i32 + 2 * margin).max(0)) as u32,
        );

        expanded_area.right() > area.left()
            && expanded_area.left() < area.right()
            && expanded_area.top() < area.bottom()
            && expanded_area.bottom() > area.top()
    }

    // pub fn collides(&self, other: &Self) -> bool {
    //     let area = match self.direction {
    //         dir::North | dir::South => Rect::new(self.area.x + 10, self.area.y, self.area.width() - 20, self.area.height()),
    //         _ => Rect::new(self.area.x, self.area.y + 10, self.area.width(), self.area.height() - 20)
    //     };

    // }
}
