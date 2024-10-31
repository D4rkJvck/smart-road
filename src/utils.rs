use crate::{
    Direction::{self, *},
    Route::{self, *},
    GAP, HEIGHT, MID_HEIGHT, MID_WIDTH, VEHICLE_HEIGHT, VEHICLE_WIDTH, WIDTH,
};

pub fn initial_position(direction: &Direction, route: &Route) -> (i32, i32) {
    match (&direction, &route) {
        //
        (North, Right) => (MID_WIDTH + GAP * 2 + 5, HEIGHT),
        (North, Straight) => (MID_WIDTH + GAP + 5, HEIGHT),
        (North, Left) => (MID_WIDTH + 5, HEIGHT),
        //
        (South, Right) => (MID_WIDTH - GAP * 2 - 45, -VEHICLE_HEIGHT),
        (South, Straight) => (MID_WIDTH - GAP - 45, -VEHICLE_HEIGHT),
        (South, Left) => (MID_WIDTH - 45, -VEHICLE_HEIGHT),
        //
        (East, Right) => (-VEHICLE_WIDTH, MID_HEIGHT + GAP * 2 + 5),
        (East, Straight) => (-VEHICLE_WIDTH, MID_HEIGHT + GAP + 5),
        (East, Left) => (-VEHICLE_WIDTH, MID_HEIGHT + 5),
        //
        (West, Right) => (WIDTH, MID_HEIGHT - GAP * 2 - 45),
        (West, Straight) => (WIDTH, MID_HEIGHT - GAP - 45),
        (West, Left) => (WIDTH, MID_HEIGHT - 45),
    }
}
