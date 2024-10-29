#[macro_export]
macro_rules! import {
    () => {
        use crate::{
            Direction::*,
            Route::{self, *},
            Vehicle, GAP, HEIGHT, MID_WIDTH,
        };
    };
}

#[macro_export]
macro_rules! add_vehicle {
    () => {{
        crate::import!();

        let direction = Direction::random();
        let route = Route::random();

        let (x, y) = match (&direction, &route) {
            (North, Right) => (MID_WIDTH + GAP * 2 + 5, HEIGHT - 45),
            _ => (0, 0),
        };

        Vehicle::new(x as i32, y as i32, direction, route)
    }};

    ($direction:expr) => {{
        crate::import!();

        let route = Route::random();

        let (x, y) = match (&$direction, &route) {
            (North, Right) => (MID_WIDTH + GAP * 2 + 5, HEIGHT - 45),
            _ => (0, 0),
        };

        Vehicle::new(x as i32, y as i32, $direction, route)
    }};
}
