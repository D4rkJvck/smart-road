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

    pub(super) fn ajust_speed(&mut self, collision_area: &Rect, others: Vec<&Vehicle>) {
        if others.iter().any(|other| self.too_close_to(other)) {
            self.speed = Speed::Stop;
            return;
        }

        if others
            .iter()
            .any(|other| self.detect_vehicle(collision_area, other))
        {
            self.speed = Speed::Stop;
            return;
        }

        // others.iter().for_each(|other| {
        //     if self.detect_vehicle(collision_area, other) {
        //         self.speed = match (
        //             other.detect_vehicle(collision_area, &self),
        //             self.has_priority_over(other),
        //         ) {
        //             (true, true) => Speed::Slow,
        //             _ => Speed::Stop,
        //         };

        //         return;
        //     }
        // });

        match self.turn_sensor {
            None => self.speed = Speed::Fast,
            Some(point) => {
                self.speed = match (
                    self.into_area(collision_area),
                    self.turned,
                    self.distance_from(point),
                ) {
                    (_, false, 1..=20) => Speed::Slow,
                    (_, false, 21..=100) => Speed::Normal,
                    _ => Speed::Fast,
                }
            }
        };
    }

    pub(super) fn pass_sensor(&mut self) {
        if let Some(next) = self.shared_sensors.first() {
            if self.has_passed_sensor(*next) {
                self.shared_sensors.reverse();
                let _ = self.shared_sensors.pop();
                self.shared_sensors.reverse();
            }
        };
    }

    pub(super) fn navigate(&mut self) {
        self.pass_sensor();

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
