use super::{
    Direction::{self, *},
    Route::{self, *},
};
use crate::{
    GAP as g, HEIGHT as h, MID_HEIGHT as m_h, MID_WIDTH as m_w, VEHICLE_HEIGHT as v_h,
    VEHICLE_WIDTH as v_w, WIDTH as w,
};

pub fn initial_position(direction: &Direction, route: &Route) -> (i32, i32) {
    match (&direction, &route) {
        //
        (North, Right) => (m_w + g * 2 + 5, h),
        (North, Straight) => (m_w + g + 5, h),
        (North, Left) => (m_w + 5, h),
        //
        (South, Right) => (m_w - g * 2 - 45, -v_h),
        (South, Straight) => (m_w - g - 45, -v_h),
        (South, Left) => (m_w - 45, -v_h),
        //
        (East, Right) => (-v_w, m_h + g * 2 + 5),
        (East, Straight) => (-v_w, m_h + g + 5),
        (East, Left) => (-v_w, m_h + 5),
        //
        (West, Right) => (w, m_h - g * 2 - 45),
        (West, Straight) => (w, m_h - g - 45),
        (West, Left) => (w, m_h - 45),
    }
}
