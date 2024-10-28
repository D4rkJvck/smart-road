use crate::{
    models::{Road, Vehicle},
    views::{Data, Window},
};

pub struct App {
    window: Window,
    road: Road,
    vehicles: Vec<Vehicle>,
}

impl App {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            window: Window::new()?,
            road: Road::new()?,
            vehicles: Vec::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.window.render(Data { road: &self.road, vehicles: &self.vehicles })?;
        self.window.listen()
    }
}
