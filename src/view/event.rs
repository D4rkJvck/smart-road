use super::Interface;
use crate::{add_vehicle, models::Direction, Vehicle};
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
                //_________________________________________
                KeyDown {
                    keycode: Some(Keycode::UP),
                    ..
                } => vehicles.push(add_vehicle!(Direction::North)),
                //__________________________________________________
                KeyDown {
                    keycode: Some(Keycode::DOWN),
                    ..
                } => vehicles.push(add_vehicle!(Direction::South)),
                //__________________________________________________
                KeyDown {
                    keycode: Some(Keycode::RIGHT),
                    ..
                } => vehicles.push(add_vehicle!(Direction::East)),
                //__________________________________________________
                KeyDown {
                    keycode: Some(Keycode::LEFT),
                    ..
                } => vehicles.push(add_vehicle!(Direction::West)),
                //__________________________________________________
                KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => vehicles.push(add_vehicle!()),
                //__________________________________
                _ => {}
            }
        }

        Ok(())
    }
}
