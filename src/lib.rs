use views::Interface;

pub mod controllers;
pub mod models;
pub mod views;

const TITLE: &str = "SMART-ROAD";
const WIDTH: u32 = 900;
const HEIGHT: u32 = 900;

macro_rules! generate {
    () => {};
}

pub struct App {
    interface: Interface,
}

impl App {
    pub fn new(interface: Interface) -> Result<Self, String> {
        Ok(Self { interface })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.interface.render()
    }
}
