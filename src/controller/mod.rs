use std::{thread, time::Duration};

use crate::{
    models::{Road, Vehicle},
    view::Interface,
    HEIGHT, TITLE, WIDTH,
};

pub struct App {
    window: Interface,
    road: Road,
    vehicles: Vec<Vehicle>,
}

impl App {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            window: Interface::new(TITLE, WIDTH, HEIGHT)?,
            road: Road::new(),
            vehicles: Vec::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            self.update()?;
            self.window.render(&self.road, &self.vehicles)?;
            self.window.listen(&mut self.vehicles)?;

            thread::sleep(Duration::from_millis(16));
        }
    }

    fn update(&mut self) -> Result<(), String> {
        self.vehicles.retain(|vehicle| vehicle.is_visible());

        self.vehicles
            .iter_mut()
            .for_each(|vehicle| vehicle.drive(&self.road.intersection));

        Ok(())
    }
}
