#[macro_export]
macro_rules! import {
    () => {
        use crate::{utils::initial_position, Category, Route, Vehicle};
    };
}

#[macro_export]
macro_rules! add_vehicle {
    () => {{
        crate::import!();

        let direction = Direction::random();
        let route = Route::random();
        let category = Category::random();
        let (x, y) = initial_position(&direction, &route);

        Vehicle::new(x as i32, y as i32, direction, route, category)
    }};

    ($direction:expr) => {{
        crate::import!();

        let route = Route::random();
        let category = Category::random();
        let (x, y) = initial_position(&$direction, &route);

        Vehicle::new(x as i32, y as i32, $direction, route, category)
    }};
}
