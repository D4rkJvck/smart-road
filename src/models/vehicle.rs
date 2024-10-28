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

pub struct Vehicle {
    speed: Speed,
    route: Route,
    sensor_range: Rect
}