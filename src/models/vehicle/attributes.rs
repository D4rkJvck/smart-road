use super::{Direction as dir, Vehicle};
use crate::{SAFETY_DISTANCE, VEHICLE_HEIGHT, VEHICLE_WIDTH};
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
        match self.direction {
            dir::North => Rect::new(
                self.area.x,
                self.area.y - SAFETY_DISTANCE,
                40,
                SAFETY_DISTANCE as u32,
            ),
            dir::East => Rect::new(
                self.area.x + VEHICLE_WIDTH,
                self.area.y,
                SAFETY_DISTANCE as u32,
                40,
            ),
            dir::South => Rect::new(
                self.area.x,
                self.area.y + VEHICLE_HEIGHT,
                40,
                SAFETY_DISTANCE as u32,
            ),
            dir::West => Rect::new(
                self.area.x - SAFETY_DISTANCE,
                self.area.y,
                SAFETY_DISTANCE as u32,
                40,
            ),
        }
    }

    pub(super) fn collidable_vehicles<'a>(&'a self, others: &Vec<&'a Self>) -> Vec<&Self> {
        others
            .iter()
            .filter(|other| {
                other
                    .shared_sensors
                    .iter()
                    .any(|sensor| self.shared_sensors.contains(sensor))
            })
            .copied()
            .collect()
    }

    pub(super) fn detect_collision<'a>(
        &'a self,
        collision_area: &Rect,
        others: Vec<&'a Vehicle>,
    ) -> Option<&Vehicle> {
        if !self.into_area(collision_area) {
            return None;
        };

        let mut collidable_vehicles = self.collidable_vehicles(&others);
        collidable_vehicles.sort_by_key(|v| self.distance_from(v.area.center()));

        for other in collidable_vehicles {
            if other.into_area(&self.sensor_range()) {
                return Some(other);
            }
        }

        None
    }
}
