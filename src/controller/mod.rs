mod stats;

use crate::{models::*, view::Interface, HEIGHT, TITLE, WIDTH};
use stats::Stats;
// use std::{thread, time::Duration};

pub struct App {
    window: Interface,
    road: Road,
    stats: Stats,
}

impl App {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            window: Interface::new(TITLE, WIDTH, HEIGHT)?,
            road: Road::new(),
            stats: Stats::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            self.update()?;
            self.window.render(&self.road)?;
            self.window.listen(&mut self.road)?;

            // thread::sleep(Duration::from_millis(16));
        }
    }

    fn update(&mut self) -> Result<(), String> {
        self.road.vehicles.retain(|vehicle| vehicle.is_visible());

        let cloned_vehicles = self.road.vehicles.clone();

        self.road.vehicles.iter_mut().for_each(|vehicle| {
            let others: Vec<&Vehicle> = cloned_vehicles
                .iter()
                .filter(|other| *other != vehicle)
                .collect();
            vehicle.drive(&self.road.intersection, &self.road.sensors, others)
        });

        Ok(())
    }
}
