use super::{
    vehicle::{
        Direction::{self, *},
        Route::{self, *},
    },
    Sensors,
};
use crate::{GAP, MID_SIZE, SIZE, VEHICLE_SIZE as v_size};
use sdl2::rect::Point;

pub(super) fn get_initial_position(direction: &Direction, route: &Route) -> (i32, i32, f32) {
    match (&direction, &route) {
        //
        (North, Right) => (MID_SIZE + GAP * 2 + 5, SIZE, 550.0),
        (North, Straight) => (MID_SIZE + GAP + 5, SIZE, 800.0),
        (North, Left) => (MID_SIZE + 5, SIZE, 850.0),
        //
        (South, Right) => (MID_SIZE - GAP * 2 - 45, -v_size, 550.0),
        (South, Straight) => (MID_SIZE - GAP - 45, -v_size, 800.0),
        (South, Left) => (MID_SIZE - 45, -v_size, 850.0),
        //
        (East, Right) => (-v_size, MID_SIZE + GAP * 2 + 5, 550.0),
        (East, Straight) => (-v_size, MID_SIZE + GAP + 5, 800.0),
        (East, Left) => (-v_size, MID_SIZE + 5, 850.0),
        //
        (West, Right) => (SIZE, MID_SIZE - GAP * 2 - 45, 550.0),
        (West, Straight) => (SIZE, MID_SIZE - GAP - 45, 800.0),
        (West, Left) => (SIZE, MID_SIZE - 45, 850.0),
    }
}

//____________________________________________________________________________________________________________
//

pub(super) fn get_shared_sensors(
    direction: &Direction,
    route: &Route,
    sensors: &Sensors,
) -> Vec<Point> {
    match (direction, route) {
        (North, Straight) => vec![sensors[4][4], sensors[4][3], sensors[4][2], sensors[4][1]],
        (North, Left) => vec![
            sensors[3][4],
            sensors[3][3],
            sensors[3][2],
            sensors[2][2],
            sensors[1][2],
        ],
        (East, Straight) => vec![sensors[1][4], sensors[2][4], sensors[3][4], sensors[4][4]],
        (East, Left) => vec![
            sensors[1][3],
            sensors[2][3],
            sensors[3][3],
            sensors[3][2],
            sensors[3][1],
        ],
        (South, Straight) => vec![sensors[1][1], sensors[1][2], sensors[1][3], sensors[1][4]],
        (South, Left) => vec![
            sensors[2][1],
            sensors[2][2],
            sensors[2][3],
            sensors[3][3],
            sensors[4][3],
        ],
        (West, Straight) => vec![sensors[4][1], sensors[3][1], sensors[2][1], sensors[1][1]],
        (West, Left) => vec![
            sensors[4][2],
            sensors[3][2],
            sensors[2][2],
            sensors[2][3],
            sensors[2][4],
        ],
        _ => vec![],
    }
}

//____________________________________________________________________________________________________________
//

pub(super) fn get_turn_sensor(
    direction: &Direction,
    route: &Route,
    sensors: &Sensors,
) -> Option<Point> {
    match (direction, route) {
        (North, Right) => Some(sensors[5][5]),
        (North, Left) => Some(sensors[3][2]),
        (East, Right) => Some(sensors[0][5]),
        (East, Left) => Some(sensors[3][3]),
        (South, Right) => Some(sensors[0][0]),
        (South, Left) => Some(sensors[2][3]),
        (West, Right) => Some(sensors[5][0]),
        (West, Left) => Some(sensors[2][2]),
        _ => None,
    }
}
