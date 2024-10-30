mod attributes;

pub use attributes::*;

use crate::{HEIGHT, VEHICLE_HEIGHT, VEHICLE_WIDTH, WIDTH};
use sdl2::rect::Rect;

pub struct Vehicle {
    pub area: Rect,
    speed: Speed,
    direction: Direction,
    route: Route,
    pub img_path: String,
    // time: ?,
    // distance: ?,
    // velocity: ?,
    // sensor_range: Rect,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, direction: Direction, route: Route) -> Self {
        Self {
            area: Rect::new(x, y, VEHICLE_WIDTH, VEHICLE_HEIGHT),
            speed: Speed::Fast,
            direction,
            route,
            img_path: String::from("./assets/red_car.png"),
            // time: (0, 0),
            // sensor_range: Rect::new(0, 0, 10, 10)
        }
    }

    /// This is the function
    /// responsible for the
    /// translation by
    /// updating the position.
    pub fn drive(&mut self) {
        self.ajust_speed();
        self.navigate();
        self.movement();
    }

    fn ajust_speed(&mut self) {
        match true {
            _ => {}
        }
    }

    fn navigate(&mut self) {}

    fn movement(&mut self) {
        match &self.direction {
            Direction::North => self.area.y -= self.velocity(),
            Direction::South => self.area.y += self.velocity(),
            Direction::East => self.area.x += self.velocity(),
            Direction::West => self.area.x -= self.velocity(),
        };
    }

    /// The velicity method gives
    /// the amount of number of
    /// pixels for the vehicle
    /// to translate.
    fn velocity(&self) -> i32 {
        match self.speed {
            Speed::Fast => 3,
            Speed::Medium => 2,
            Speed::Slow => 1,
            Speed::Stop => 0,
        }
    }

    pub fn angle(&self) -> f64 {
        match self.direction {
            Direction::North => 0.0,
            Direction::East => 90.0,
            Direction::South => 180.0,
            Direction::West => 270.0,
        }
    }

    /// This function is crucial when
    /// it comes to remove vehicule
    /// as it confirm that the
    /// vehicle is out of the window.
    pub fn is_visible(&self) -> bool {
        self.area.top() < HEIGHT as i32
            && self.area.left() < WIDTH as i32
            && self.area.right() > 0
            && self.area.bottom() > 0
    }
}
