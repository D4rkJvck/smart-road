mod controller;
mod models;
mod view;

pub use controller::App;
pub use models::{Direction, Speed, Vehicle};

const TITLE: &str = "SMART-ROAD";
const SIZE: i32 = 800;
const MID_SIZE: i32 = SIZE / 2;
const GAP: i32 = 50;
const VEHICLE_SIZE: i32 = 40;
const SAFETY_DISTANCE: i32 = 150;
