use super::{Direction, Speed, Vehicle};

impl Vehicle {
    pub fn slow_down(&mut self) {
        self.speed = match self.speed {
            Speed::Fast => Speed::Medium,
            Speed::Medium => Speed::Slow,
            _ => Speed::Stop,
        }
    }

    pub fn speed_up(&mut self) {
        self.speed = match self.speed {
            Speed::Stop => Speed::Slow,
            Speed::Slow => Speed::Medium,
            _ => Speed::Fast,
        }
    }

        
    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

}