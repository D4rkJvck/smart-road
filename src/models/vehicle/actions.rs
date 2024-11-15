use super::{Direction as dir, Route, Vehicle};
use crate::TIME;
use sdl2::rect::Rect;

impl Vehicle {
    pub(super) fn movement(&mut self) {
        // let speed = Speed::velocity(&self.speed);
        match &self.direction {
            dir::North => self.area.y -= self.speed,
            dir::South => self.area.y += self.speed,
            dir::East => self.area.x += self.speed,
            dir::West => self.area.x -= self.speed,
        };
    }

    pub(super) fn ajust_speed(&mut self, collision_area: &Rect, others: Vec<&Vehicle>) {
        if others.iter().any(|other| self.too_close_to(other)) {
            self.speed = 0;
            return;
        }

        if let Some(v) = self.detect_collision(collision_area, others) {
            self.speed = self.distance_from(v.area.center()) / TIME;
            return;
        };

        match self.turn_sensor {
            None => self.speed = self.distance / TIME,
            Some(point) => {
                self.speed = match (self.turned, self.distance_from(point)) {
                    (false, 1..=20) => 1,
                    _ => self.distance / TIME,
                }
            }
        };
    }

    pub(super) fn pass_sensor(&mut self) {
        if let Some(next) = self.shared_sensors.first() {
            if self.passed_sensor(*next) {
                self.shared_sensors.reverse();
                let _ = self.shared_sensors.pop();
                self.shared_sensors.reverse();
            }
        };
    }

    pub(super) fn navigate(&mut self) {
        let turning_point = match self.turn_sensor {
            Some(point) => point,
            None => return,
        };

        if self.turned || self.area.center() != turning_point {
            return;
        };

        match self.route {
            Route::Right => self.turn_right(),
            Route::Left => self.turn_left(),
            Route::Straight => {}
        };

        self.pass_sensor();
    }

    fn turn_right(&mut self) {
        self.turned = true;

        self.direction = match self.direction {
            dir::North => dir::East,
            dir::East => dir::South,
            dir::South => dir::West,
            dir::West => dir::North,
        };
    }

    fn turn_left(&mut self) {
        self.turned = true;

        self.direction = match self.direction {
            dir::North => dir::West,
            dir::East => dir::North,
            dir::South => dir::East,
            dir::West => dir::South,
        };
    }
}
