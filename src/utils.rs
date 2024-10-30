use crate::{Direction::{self, *}, Route::{self, *}, GAP, HEIGHT, MID_WIDTH};

pub fn initial_position(direction: &Direction, route: &Route) -> (u32, u32) {
    match (&direction, &route) {
        (North, Right) => (MID_WIDTH + GAP * 2 + 5, HEIGHT - 85),
        (North, Straight) => (MID_WIDTH + GAP + 5, HEIGHT - 85),
        (North, Left) => (MID_WIDTH + 5, HEIGHT - 85),
        (South, Right) => (MID_WIDTH - GAP * 2 - 5, 85),
        (South, Straight) => (MID_WIDTH - GAP - 5, 85),
        (South, Left) => (MID_WIDTH - 5, 85),
        _ => (0, 0),
    }
}

//___________________________________________________________________________________
//

pub fn apply_texture() {
    let texture = 
}