mod event;
mod stats;

// use std::{thread, time::Duration};

use crate::{models::*, view::Interface, SIZE, TITLE};
pub use stats::Stats;

pub struct App {
    window: Interface,
    intersection: Intersection,
    stats: Stats,
}

impl App {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            window: Interface::new(TITLE, SIZE, SIZE)?,
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

            // thread::sleep(Duration::from_millis(8));
        }

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

            vehicle.drive(&self.intersection.collision_area, others, &mut self.stats)
        });

        Ok(self.stats.update(&self.intersection.vehicles))
    }
}
