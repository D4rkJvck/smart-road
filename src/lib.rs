mod macros;

mod controller;
mod model;
mod view;

pub use controller::App;

const TITLE: &str = "SMART-ROAD";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const MID_WIDTH: u32 = WIDTH / 2;
const MID_HEIGHT: u32 = HEIGHT / 2;
const GAP: u32 = 40;

const SAFETY_DISTANCE: u32 = 100;

