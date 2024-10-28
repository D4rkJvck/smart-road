mod macros;

mod controllers;
mod models;
mod views;

pub use controllers::App;

const TITLE: &str = "SMART-ROAD";
const WIDTH: u32 = 900;
const HEIGHT: u32 = 900;

const SAFETY_DISTANCE: u32 = 100;

