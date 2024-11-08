use sdl2::rect::Rect;

#[test]
fn test_vehicle_state_into_area() {
    let area = Rect::new(350, 350, 50, 50);

    let vehicles = vec![
        Vehicle::new(
            311,
            311,
            Direction::random(),
            Route::random(),
            Category::random(),
        ),
        Vehicle::new(
            360,
            360,
            Direction::random(),
            Route::random(),
            Category::random(),
        ),
    ];

    assert!(vehicles.iter().all(|vehicle| vehicle.into_area(&area)))
}

#[test]
#[should_panic]
fn test_vehicle_state_not_into_area() {
    let area = Rect::new(350, 350, 50, 50);

    let vehicles = vec![
        Vehicle::new(
            310,
            311,
            Direction::random(),
            Route::random(),
            Category::random(),
        ),
        Vehicle::new(
            311,
            310,
            Direction::random(),
            Route::random(),
            Category::random(),
        ),
    ];

    assert!(vehicles.iter().any(|vehicle| vehicle.into_area(&area)))
}
