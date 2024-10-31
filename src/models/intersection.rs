use sdl2::rect::Rect;

use crate::{GAP, MID_HEIGHT, MID_WIDTH};

#[derive(Clone, Copy)]
pub struct Intersection {
    pub area: Rect,
    vehicle_count: u32,
    max_speed: u32,
    min_speed: u32,
    max_time: f32,
    min_time: f32,
    collision_count: u32,
}

impl Intersection {
    pub fn new() -> Self {
        Self {
            area: Rect::new(
                MID_WIDTH - GAP * 3,
                MID_HEIGHT - GAP * 3,
                GAP as u32 * 6 + 1,
                GAP as u32 * 6 + 1,
            ),
            vehicle_count: 0,
            max_speed: 0,
            min_speed: 0,
            max_time: 0.0,
            min_time: 0.0,
            collision_count: 0,
        }
    }
}
