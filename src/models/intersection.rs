use sdl2::rect::Rect;

pub struct Intersection {
    area: Rect,
    vehicle_count: u32,
    max_speed: u32,
    min_speed: u32,
    max_time: f32,
    min_time: f32,
    collision_count: u32,
}

impl Intersection {
    pub fn new() -> Self {
        Intersection {
            area: Rect::new(0, 0, 0, 0),
            vehicle_count: 0,
            max_speed: 0,
            min_speed: 0,
            max_time: 0.0,
            min_time: 0.0,
            collision_count: 0,
        }
    }
}
