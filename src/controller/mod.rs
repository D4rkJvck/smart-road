mod event;
mod stats;

use crate::{models::*, view::Interface, HEIGHT, TITLE, WIDTH};
pub use stats::Stats;

pub struct App {
    window: Interface,
    intersection: Intersection,
    stats: Stats,
}

impl App {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            window: Interface::new(TITLE, WIDTH, HEIGHT)?,
            intersection: Intersection::new(),
            stats: Stats::new(),
        })
    }

    pub async fn run(&mut self) -> Result<(), String> {
        'simulation: loop {
            self.update()?;
            self.window.render(&self.intersection)?;
            if let Err(_) = self.listen() {
                break 'simulation;
            }
        }

        self.window
            .canvas
            .window_mut()
            .set_size(WIDTH as u32 / 2, HEIGHT as u32 / 2)
            .map_err(|err| format!("Resize! -> {}", err))?;

        self.window.display_stats(self.stats.get())?;

        loop {
            self.listen()?;
        }
    }

    fn update(&mut self) -> Result<(), String> {
        self.intersection
            .vehicles
            .retain(|vehicle| vehicle.is_visible());

        if self.intersection.auto_spawn {
            self.intersection.new_vehicle(Direction::random());
        }

        let cloned_vehicles = self.intersection.vehicles.clone();

        self.intersection.vehicles.iter_mut().for_each(|vehicle| {
            let others = cloned_vehicles
                .iter()
                .filter(|other| {
                    *other != vehicle
                        && vehicle
                            .shared_sensors
                            .iter()
                            .any(|sensor| other.shared_sensors.contains(sensor))
                })
                .collect();

            vehicle.drive(&self.intersection.collision_area, others)
        });

        Ok(self.stats.update(&self.intersection.vehicles))
    }
}
