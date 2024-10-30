use crate::{Direction::{self, *}, Route::{self, *}, GAP, HEIGHT, MID_WIDTH};

pub fn initial_position(direction: &Direction, route: &Route) -> (u32, u32) {
    match (&direction, &route) {
        (North, Right) => (MID_WIDTH + GAP * 2 + 10, HEIGHT - 85),
        (North, Straight) => (MID_WIDTH + GAP + 10, HEIGHT - 85),
        (North, Left) => (MID_WIDTH + 10, HEIGHT - 85),
        (South, Right) => (MID_WIDTH - GAP * 2 - 40, 5),
        (South, Straight) => (MID_WIDTH - GAP - 40, 5),
        (South, Left) => (MID_WIDTH - 40, 5),
        _ => (0, 0),
    }
}
