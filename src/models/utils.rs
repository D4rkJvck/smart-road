use sdl2::rect::Point;

use super::{
    Direction::{self, *},
    Route::{self, *},
    Sensors,
};
use crate::{
    GAP as g, HEIGHT as h, MID_HEIGHT as m_h, MID_WIDTH as m_w, VEHICLE_HEIGHT as v_h,
    VEHICLE_WIDTH as v_w, WIDTH as w,
};

pub fn get_initial_position(direction: &Direction, route: &Route) -> (i32, i32, i32) {
    match (&direction, &route) {
        //
        (North, Right) => (m_w + g * 2 + 5, h, 300),
        (North, Straight) => (m_w + g + 5, h, 550),
        (North, Left) => (m_w + 5, h, 600),
        //
        (South, Right) => (m_w - g * 2 - 45, -v_h, 300),
        (South, Straight) => (m_w - g - 45, -v_h, 550),
        (South, Left) => (m_w - 45, -v_h, 600),
        //
        (East, Right) => (-v_w, m_h + g * 2 + 5, 300),
        (East, Straight) => (-v_w, m_h + g + 5, 550),
        (East, Left) => (-v_w, m_h + 5, 600),
        //
        (West, Right) => (w, m_h - g * 2 - 45, 300),
        (West, Straight) => (w, m_h - g - 45, 550),
        (West, Left) => (w, m_h - 45, 600),
    }
}

//____________________________________________________________________________________________________________
//

pub fn get_shared_sensors(direction: &Direction, route: &Route, sensors: &Sensors) -> Vec<Point> {
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

pub fn get_turn_sensor(direction: &Direction, route: &Route, sensors: &Sensors) -> Option<Point> {
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
