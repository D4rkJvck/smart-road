use std::{thread, time::Duration};

use crate::{models::Road, view::Interface, HEIGHT, TITLE, WIDTH};

pub struct App {
    window: Interface,
    road: Road,
}

impl App {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            window: Interface::new(TITLE, WIDTH, HEIGHT)?,
            road: Road::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            self.update()?;
            self.window.render(&self.road.vehicles)?;
            self.window.listen(&mut self.road.vehicles)?;

            thread::sleep(Duration::from_millis(16));
        }
    }

    fn update(&mut self) -> Result<(), String> {
        self.road.vehicles.retain(|vehicle| vehicle.is_visible());

        self.road
            .vehicles
            .iter_mut()
            .for_each(|vehicle| vehicle.drive(&self.road.intersection, &self.road.sensors));

        Ok(())
    }
}
