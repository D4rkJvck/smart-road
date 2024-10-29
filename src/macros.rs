#[macro_export]
macro_rules! import {
    () => {
        use crate::{
            Route,
            Vehicle,
            utils,
        };
    };
}

#[macro_export]
macro_rules! add_vehicle {
    () => {{
        crate::import!();

        let direction = Direction::random();
        let route = Route::random();

        let (x, y) = utils::position(&direction, &route);

        Vehicle::new(x as i32, y as i32, direction, route)
    }};

    ($direction:expr) => {{
        crate::import!();

        let route = Route::random();

        let (x, y) = utils::position(&$direction, &route);

        Vehicle::new(x as i32, y as i32, $direction, route)
    }};
}
