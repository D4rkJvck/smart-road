use sdl2::rect::Rect;

enum Speed {
    Stop,
    Slow,
    Medium,
    Fast,
}

enum Route {
    Right,
    Left,
    Straight,
}

enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Vehicle {
    area: Rect,
    speed: Speed,
    direction: Direction,
    route: Route,
    // sensor_range: Rect,
}

impl Vehicle {
    pub fn new(direction: Direction, route: Route) -> Self {
        use Direction::*;
        use Route::*;

        let area = match (&direction, &route) {
            _ => Rect::new(0, 0, 0, 0),
        };

        Self {
            area,
            speed: Speed::Fast,
            direction,
            route,
            // sensor_range,
        }
    }
}
