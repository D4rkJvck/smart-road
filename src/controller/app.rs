use crate::{
    model::{Road, Vehicle},
    view::Interface,
};

pub struct App {
    window: Interface,
    road: Road,
    vehicles: Vec<Vehicle>,
}

impl App {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            window: Interface::new()?,
            road: Road::new(),
            vehicles: Vec::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.update()?;
        self.window.render(&self.road, &self.vehicles)?;
        self.window.listen()
    }

    fn update(&mut self) -> Result<(), String> {
        Ok(())
    }
}
