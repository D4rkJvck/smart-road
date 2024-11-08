use smart_road::{Category, Direction, Route, Vehicle};

#[test]
fn test_vehicle_state_is_visible() {
    let vehicles = vec![
        Vehicle::new(
            800,
            800,
            Direction::random(),
            Route::random(),
            Category::random(),
        ),
        Vehicle::new(
            400,
            400,
            Direction::random(),
            Route::random(),
            Category::random(),
        ),
    ];

    assert!(vehicles.iter().all(|vehicle| vehicle.is_visible()))
}

#[test]
#[should_panic]
fn test_vehicle_state_not_visible() {
    let vehicles = vec![
        Vehicle::new(
            801,
            80,
            Direction::random(),
            Route::random(),
            Category::random(),
        ),
        Vehicle::new(
            400,
            801,
            Direction::random(),
            Route::random(),
            Category::random(),
        ),
    ];

    assert!(vehicles.iter().any(|vehicle| vehicle.is_visible()))
}
