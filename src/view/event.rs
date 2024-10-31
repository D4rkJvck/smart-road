use super::Interface;
use crate::{add_vehicle, models::Direction, Vehicle};
use sdl2::{
    event::Event::{KeyDown, Quit},
    keyboard::Keycode as k,
};

impl Interface {
    pub fn listen(&mut self, vehicles: &mut Vec<Vehicle>) -> Result<(), String> {
        let events = self.event_pump.poll_iter();

        for event in events {
            match event {
                Quit { .. }
                | KeyDown {
                    keycode: Some(k::ESCAPE),
                    ..
                } => return Err("Exiting...".to_string()),
                //_________________________________________
                KeyDown {
                    keycode: Some(k::UP),
                    ..
                } => vehicles.push(add_vehicle!(Direction::North)),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::DOWN),
                    ..
                } => vehicles.push(add_vehicle!(Direction::South)),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::RIGHT),
                    ..
                } => vehicles.push(add_vehicle!(Direction::East)),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::LEFT),
                    ..
                } => vehicles.push(add_vehicle!(Direction::West)),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::R),
                    ..
                } => vehicles.push(add_vehicle!()),
                //__________________________________
                _ => {}
            }
        }

        Ok(())
    }
}
