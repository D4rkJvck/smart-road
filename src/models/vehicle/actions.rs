use sdl2::rect::Rect;

use super::{Direction, Route, Speed, Vehicle};

impl Vehicle {
    pub fn movement(&mut self) {
        let speed = self.velocity();

        match &self.direction {
            Direction::North => self.area.y -= speed,
            Direction::South => self.area.y += speed,
            Direction::East => self.area.x += speed,
            Direction::West => self.area.x -= speed,
        };
    }

    pub fn slow_down(&mut self) {
        self.speed = match self.speed {
            Speed::Fast => Speed::Normal,
            Speed::Normal => Speed::Slow,
            _ => self.speed,
        }
    }

    pub fn speed_up(&mut self) {
        self.speed = match self.speed {
            Speed::Stop => Speed::Slow,
            Speed::Slow => Speed::Normal,
            _ => Speed::Fast,
        }
    }

    pub fn navigate(&mut self, sensors: &[[Rect; 6]; 6]) {
        let turning_point = match self.turning_point(sensors) {
            Some(point) => point,
            None => return
        };

        if self.crossed || self.area.center() != turning_point {
            return
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
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn turn_left(&mut self) {
        self.crossed = true;

        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
    }
}
