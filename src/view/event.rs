use super::Interface;
use crate::{
    add_vehicle,
    models::{Direction, Road},
    Vehicle,
};
use sdl2::{
    event::Event::{KeyDown, Quit},
    keyboard::Keycode,
};

impl Interface {
    pub fn listen(&mut self, vehicles: &mut Vec<Vehicle>) -> Result<(), String> {
        let events = self.event_pump.poll_iter();

        for event in events {
            match event {
                Quit { .. }
                | KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => return Err("Exiting...".to_string()),

                // Generate a vehicle from South to North
                KeyDown {
                    keycode: Some(Keycode::UP),
                    ..
                } => vehicles.push(add_vehicle!(Direction::North)),

                // Generate a vehicle from North to South
                KeyDown {
                    keycode: Some(Keycode::DOWN),
                    ..
                } => vehicles.push(add_vehicle!(Direction::South)),

                // Generate a vehicle from random direction
                KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => vehicles.push(add_vehicle!()),

                _ => {}
            }
        }

        Ok(())
    }
}
