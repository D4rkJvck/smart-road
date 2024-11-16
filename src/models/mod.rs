mod utils;
mod vehicle;

use crate::{GAP, MID_HEIGHT, MID_WIDTH};
use sdl2::rect::{Point, Rect};
use utils::{get_shared_sensors, get_turn_sensor};
use vehicle::Route;
pub use vehicle::{Direction, Vehicle};

pub type Sensors = [[Point; 6]; 6];

pub struct Road {
    pub intersection: Rect,
    pub collision_area: Rect,
    pub sensors: Sensors,
    pub vehicles: Vec<Vehicle>,
}

impl Road {
    pub fn new() -> Self {
        let intersection = Rect::new(
            MID_WIDTH - GAP * 3,
            MID_HEIGHT - GAP * 3,
            GAP as u32 * 6,
            GAP as u32 * 6,
        );

        let collision_area = Rect::new(
            intersection.x + GAP,
            intersection.y + GAP,
            intersection.width() - GAP as u32 * 2,
            intersection.height() - GAP as u32 * 2,
        );

        let mut sensors: Sensors = [[Point::new(0, 0); 6]; 6];

        for x in 0..6 {
            for y in 0..6 {
                sensors[x][y] = Rect::new(
                    intersection.x + GAP * x as i32,
                    intersection.y + GAP * y as i32,
                    GAP as u32,
                    GAP as u32,
                )
                .center()
            }
        }

        Self {
            intersection,
            collision_area,
            sensors,
            vehicles: Vec::new(),
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

        self.vehicles
            .push(Vehicle::new(direction, route, shared_sensors, turn_sensor));
    }
}
