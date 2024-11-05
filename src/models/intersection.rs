use sdl2::rect::Rect;

use crate::{GAP, MID_HEIGHT, MID_WIDTH};

pub struct Intersection {
    pub area: Rect,
    pub sensors: [[Rect; 6]; 6],
    // vehicle_count: u32,
    // max_speed: u32,
    // min_speed: u32,
    // max_time: f32,
    // min_time: f32,
    // collision_count: u32,
}

impl Intersection {
    pub fn new() -> Self {
        let area = Rect::new(
            MID_WIDTH - GAP * 3,
            MID_HEIGHT - GAP * 3,
            GAP as u32 * 6 + 1,
            GAP as u32 * 6 + 1,
        );

        let mut sensors = [[Rect::new(0, 0, 0, 0); 6]; 6];

        for x in 0..6 {
            for y in 0..6 {
                sensors[x][y] = Rect::new(
                    area.x + GAP * x as i32,
                    area.y + GAP * y as i32,
                    GAP as u32,
                    GAP as u32,
                )
            }
        }

        Self {
            area,
            sensors,
            // vehicle_count: 0,
            // max_speed: 0,
            // min_speed: 0,
            // max_time: 0.0,
            // min_time: 0.0,
            // collision_count: 0,
        }
    }
}
