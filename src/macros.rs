#[macro_export]
macro_rules! import {
    () => {
        use crate::{utils::initial_position, Category, Route, Vehicle};
    };
}

//___________________________________________________________________________
//

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

//___________________________________________________________________________
//

// #[macro_export]
// macro_rules! texturize {
//     () => {
//         let path = match Category::random() {
//             Category::Police => "./assets/cars/police.png",
//             Category::Taxi => "./assets/cars/taxi.png",
//             Category::Black => "./assets/cars/black.png",
//             Category::Red => "./assets/cars/red.png",
//         }

//         // let
//     };
// }
