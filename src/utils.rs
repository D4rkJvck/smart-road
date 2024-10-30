use crate::{
    Direction::{self, *},
    Route::{self, *},
    GAP, HEIGHT, MID_HEIGHT, MID_WIDTH, WIDTH,
};

pub fn initial_position(direction: &Direction, route: &Route) -> (u32, u32) {
    match (&direction, &route) {
        //
        (North, Right) => (MID_WIDTH + GAP * 2 + 5, HEIGHT - 85),
        (North, Straight) => (MID_WIDTH + GAP + 5, HEIGHT - 85),
        (North, Left) => (MID_WIDTH + 5, HEIGHT - 85),
        //
        (South, Right) => (MID_WIDTH - GAP * 2 - 45, 5),
        (South, Straight) => (MID_WIDTH - GAP - 45, 5),
        (South, Left) => (MID_WIDTH - 45, 5),
        //
        (East, Right) => (5, MID_HEIGHT + GAP * 2 + 5),
        (East, Straight) => (5, MID_HEIGHT + GAP + 5),
        (East, Left) => (5, MID_HEIGHT + 5),
        //
        (West, Right) => (WIDTH - 85, MID_HEIGHT - GAP * 2 - 45),
        (West, Straight) => (WIDTH - 85, MID_HEIGHT - GAP - 45),
        (West, Left) => (WIDTH - 85, MID_HEIGHT - 45),
    }
}
