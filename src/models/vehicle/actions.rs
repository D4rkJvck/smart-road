use super::{Direction as dir, Route, Vehicle};

impl Vehicle {
    pub fn movement(&mut self) {
        // let speed = Speed::velocity(&self.speed);
        match &self.direction {
            dir::North => self.area.y -= self.speed,
            dir::South => self.area.y += self.speed,
            dir::East => self.area.x += self.speed,
            dir::West => self.area.x -= self.speed,
        };
    }

    pub fn navigate(&mut self) {
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
        }
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
