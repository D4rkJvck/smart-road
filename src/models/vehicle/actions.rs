use super::{Direction as dir, Route, Speed, Vehicle};
use crate::models::SensorGrid;

impl Vehicle {
    pub fn movement(&mut self) {
        let speed = Speed::velocity(&self.speed);

        match &self.direction {
            dir::North => self.area.y -= speed,
            dir::South => self.area.y += speed,
            dir::East => self.area.x += speed,
            dir::West => self.area.x -= speed,
        };
    }

    pub fn slow_down(&mut self) {
        self.speed = match self.speed {
            Speed::Fast => Speed::Normal,
            Speed::Normal => Speed::Slow,
            _ => Speed::Stop,
        }
    }

    pub fn speed_up(&mut self) {
        self.speed = match self.speed {
            Speed::Stop => Speed::Slow,
            Speed::Slow => Speed::Normal,
            _ => Speed::Fast,
        }
    }

    pub fn navigate(&mut self, sensors: &SensorGrid) {
        let turning_point = match self.turning_point(sensors) {
            Some(point) => point,
            None => return,
        };

        if self.crossed || self.area.center() != turning_point {
            return;
        };

        match self.route {
            Route::Right => self.turn_right(),
            Route::Left => self.turn_left(),
            Route::Straight => self.speed_up(),
        }
    }

    fn turn_right(&mut self) {
        self.crossed = true;

        self.direction = match self.direction {
            dir::North => dir::East,
            dir::East => dir::South,
            dir::South => dir::West,
            dir::West => dir::North,
        };
    }

    fn turn_left(&mut self) {
        self.crossed = true;

        self.direction = match self.direction {
            dir::North => dir::West,
            dir::East => dir::North,
            dir::South => dir::East,
            dir::West => dir::South,
        };
    }
}
