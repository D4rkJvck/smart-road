mod statistics;

use crate::{models::*, view::Interface, HEIGHT, TITLE, WIDTH};
pub use statistics::Statistics;
use std::{thread, time::Duration};

pub struct App {
    window: Interface,
    intersection: Intersection,
    statistics: Statistics,
}

impl App {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            window: Interface::new(TITLE, WIDTH, HEIGHT)?,
            intersection: Intersection::new(),
            statistics: Statistics::new(),
        })
    }

    pub async fn run(&mut self) -> Result<(), String> {
        'simulation: loop {
            self.update()?;
            self.window.render(&self.intersection)?;
            if let Err(_) = self.window.listen(&mut self.intersection) {
                break 'simulation;
            }
        }

        thread::sleep(Duration::from_millis(250));

        self.window.display_stats(&self.statistics);

        'stats: loop {
            if let Err(_) = self.window.listen(&mut self.intersection) {
                break 'stats;
            }
        }

        Ok(())
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

        Ok(())
    }
}
