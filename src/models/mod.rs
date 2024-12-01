mod utils;
mod vehicle;

use crate::{GAP, MID_SIZE};
use sdl2::rect::{Point, Rect};
use utils::{get_shared_sensors, get_turn_sensor};
use vehicle::Route;
pub use vehicle::Speed;
pub use vehicle::{Direction, Vehicle};

pub type Sensors = [[Point; 6]; 6];

pub struct Intersection {
    pub collision_area: Rect,
    pub sensors: Sensors,
    pub vehicles: Vec<Vehicle>,
    pub sensor_visibility: bool,
    pub auto_spawn: bool,
}

impl Intersection {
    pub fn new() -> Self {
        let center = Rect::new(
            MID_SIZE - GAP * 3,
            MID_SIZE - GAP * 3,
            GAP as u32 * 6,
            GAP as u32 * 6,
        );

        let collision_area = Rect::new(
            center.x + GAP,
            center.y + GAP,
            center.width() - GAP as u32 * 2,
            center.height() - GAP as u32 * 2,
        );

        let mut sensors: Sensors = [[Point::new(0, 0); 6]; 6];

        for x in 0..6 {
            for y in 0..6 {
                sensors[x][y] = Rect::new(
                    center.x + GAP * x as i32,
                    center.y + GAP * y as i32,
                    GAP as u32,
                    GAP as u32,
                )
                .center()
            }
        }

        Self {
            collision_area,
            sensors,
            vehicles: Vec::new(),
            sensor_visibility: false,
            auto_spawn: false,
        }
    }

    pub fn new_vehicle(&mut self, direction: Direction) {
        let route = Route::random();

        if self
            .vehicles
            .iter()
            .any(|other| Vehicle::new(direction, route, vec![], None).too_close_to(other))
        {
            return;
        };

        let shared_sensors = get_shared_sensors(&direction, &route, &self.sensors);
        let turn_sensor = get_turn_sensor(&direction, &route, &self.sensors);

        if self.vehicles.len() < 12 {
            self.vehicles
                .push(Vehicle::new(direction, route, shared_sensors, turn_sensor));
        }
    }
}
