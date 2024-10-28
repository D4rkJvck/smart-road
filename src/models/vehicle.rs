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
    speed: Speed,
    route: Route,
    direction: Direction,
    sensor_range: Rect
}