use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]

pub enum Speed {
    Stop,
    Slow,
    Normal,
    Fast,
}

//_____________________________________________
//

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..4) {
            0 => Self::North,
            1 => Self::South,
            2 => Self::East,
            _ => Self::West,
        }
    }
}

//_____________________________________________
//

#[derive(Clone, Copy, PartialEq)]
pub enum Route {
    Right,
    Left,
    Straight,
}

impl Route {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..3) {
            0 => Self::Left,
            1 => Self::Straight,
            _ => Self::Right,
        }
    }
}
