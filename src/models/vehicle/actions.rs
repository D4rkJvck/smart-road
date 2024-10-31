use super::{Direction, Route, Speed, Vehicle};
use crate::{GAP as g, MID_HEIGHT as m_h, MID_WIDTH as m_w};

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

    pub fn navigate(&mut self) {
        match self.route {
            Route::Right => self.turn_right(),
            Route::Left => self.turn_left(),
            _ => self.speed_up(),
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North if self.area.top() == m_h + g * 2 + 5 => Direction::East,
            Direction::East if self.area.right() == m_w - g * 2 - 5 => Direction::South,
            Direction::South if self.area.bottom() == m_h - g * 2 - 5 => Direction::West,
            Direction::West if self.area.left() == m_w + g * 2 + 5 => Direction::North,
            _ => self.direction,
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::North if self.area.top() == m_h - g + 5 => Direction::West,
            Direction::East if self.area.right() == m_w + g - 5 => Direction::North,
            Direction::South if self.area.bottom() == m_h + g - 5 => Direction::East,
            Direction::West if self.area.left() == m_w - g + 5 => Direction::South,
            _ => self.direction
        };
    }
}
