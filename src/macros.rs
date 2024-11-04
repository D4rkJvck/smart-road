#[macro_export]
macro_rules! import {
    () => {
        use crate::{utils::initial_position, Route, Vehicle};
    };
}

#[macro_export]
macro_rules! add_vehicle {
    () => {{
        crate::import!();

        let direction = Direction::random();
        let route = Route::random();
        let (x, y) = initial_position(&direction, &route);

        Vehicle::new(x as i32, y as i32, direction, route, false)
    }};

    ($direction:expr) => {{
        crate::import!();

        let route = Route::random();
        let (x, y) = initial_position(&$direction, &route);

        Vehicle::new(x as i32, y as i32, $direction, route, false)
    }};
    // Cas avec direction et priorité spécifiées
    ($direction:expr, $priority:expr) => {{
        crate::import!();

        let route = Route::random();
        let (x, y) = initial_position(&$direction, &route);

        Vehicle::new(x as i32, y as i32, $direction, route, $priority)
    }};
}
