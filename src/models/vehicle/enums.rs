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
        match thread_rng().gen_range(0..5) {
            0 => Self::Left,
            1 | 2 => Self::Straight,
            _ => Self::Right,
        }
    }
}

//_____________________________________________
//

#[derive(Clone, Copy)]
pub enum Category {
    Police,
    Taxi,
    Red,
    Black,
}

impl Category {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..11) {
            0 => Self::Police,
            1..=5 => Self::Taxi,
            6..=8 => Self::Black,
            _ => Self::Red,
        }
    }
}
