mod navigation;

pub use navigation::*;

use crate::{HEIGHT, VEHICLE_HEIGHT, VEHICLE_WIDTH, WIDTH};
use sdl2::rect::Rect;

pub struct Vehicle {
    pub area: Rect,
    speed: Speed,
    direction: Direction,
    route: Route,
    pub texture: String,
    // time_interval: (u64, u64),
    // sensor_range: Rect,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, direction: Direction, route: Route) -> Self {
        Self {
            area: Rect::new(x, y, VEHICLE_WIDTH, VEHICLE_HEIGHT),
            speed: Speed::Fast,
            direction,
            route,
            texture: String::from("./assets/red_car.png"),
            // time_interval: (0, 0),
            // sensor_range: Rect::new(0, 0, 10, 10)
        }
    }

    /// This is the function
    /// responsible for the
    /// translation by
    /// updating the position.
    pub fn drive(&mut self) {
        use Direction::*;
        use Route::*;
        use Speed::*;

        match (&self.direction, &self.speed, &self.route) {
            (North, Fast, Straight) => self.area.y -= 3,
            (North, Fast, Right) => self.area.y -= 3,
            _ => {}
        };
    }

    pub fn moving(&mut self) {}

    // pub fn ajust_speed(&mut self) {
    //     match true {
    //         self.is_near_intersection() =>
    //         _ => {}
    //     }
    // }

    /// This function is crucial when
    /// it comes to remove vehicule
    /// as it confirm that the vehicle is
    /// out of the window...
    pub fn is_visible(&self) -> bool {
        self.area.top() < HEIGHT as i32
            && self.area.left() < WIDTH as i32
            && self.area.right() > 0
            && self.area.bottom() > 0
    }
}
