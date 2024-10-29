use crate::{Direction::{self, *}, Route::{self, *}, GAP, HEIGHT, MID_WIDTH};

pub fn position(direction: &Direction, route: &Route) -> (u32, u32) {
    match (&direction, &route) {
        (North, Right) => (MID_WIDTH + GAP * 2 + 5, HEIGHT - 45),
        _ => (0, 0),
    }
}