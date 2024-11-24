use crate::controller::Stats;

use super::{Direction as dir, Route, Speed, Vehicle};
use sdl2::rect::Rect;

impl Vehicle {
    pub(super) fn movement(&mut self) {
        let speed = Speed::velocity(&self.speed);

        match &self.direction {
            dir::North => self.area.y -= speed,
            dir::South => self.area.y += speed,
            dir::East => self.area.x += speed,
            dir::West => self.area.x -= speed,
        };
    }

    pub(super) fn ajust_speed(
        &mut self,
        collision_area: &Rect,
        others: Vec<&Vehicle>,
        stats: &mut Stats,
    ) {
        if others.iter().any(|other| self.too_close_to(other)) {
            stats.close_calls += 1;
            self.speed = Speed::Stop;
            return;
        };

        match self.turn_sensor {
            None => self.speed = Speed::Fast,
            Some(point) => {
                self.speed = match self.distance_from(point) {
                    1..=3 => Speed::Slow,
                    4..=50 => Speed::Normal,
                    _ => Speed::Fast,
                }
            }
        };

        others.iter().for_each(|other| {
            if self.detect_vehicle(collision_area, other) {
                match (
                    other.detect_vehicle(collision_area, &self),
                    self.has_priority_over(other),
                ) {
                    (true, true) => {
                        stats.priority_calls += 1;
                        self.speed = Speed::Slow;
                    }
                    _ => self.speed = Speed::Stop,
                };
            }
        });
    }

    pub(super) fn navigate(&mut self) {
        if let Some(turning_point) = self.turn_sensor {
            if self.turned || self.area.center() != turning_point {
                return;
            }

            self.turned = true;

            match self.route {
                Route::Right => self.turn_right(),
                Route::Left => self.turn_left(),
                Route::Straight => {}
            }
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            dir::North => dir::East,
            dir::East => dir::South,
            dir::South => dir::West,
            dir::West => dir::North,
        };
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            dir::North => dir::West,
            dir::East => dir::North,
            dir::South => dir::East,
            dir::West => dir::South,
        };
    }
}
