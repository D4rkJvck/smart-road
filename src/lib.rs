mod controller;
mod models;
mod view;

pub use controller::App;
pub use models::{Category, Direction, Route, Vehicle};

const TITLE: &str = "SMART-ROAD";
const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const MID_WIDTH: i32 = WIDTH / 2;
const MID_HEIGHT: i32 = HEIGHT / 2;
const GAP: i32 = 50;
const VEHICLE_WIDTH: i32 = 40;
const VEHICLE_HEIGHT: i32 = 40;
const SAFETY_DISTANCE: i32 = 160;
const TIME: i32 = 300;
