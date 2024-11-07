use super::Interface;
use crate::models::{Direction, Road};
use sdl2::{
    event::Event::{KeyDown, Quit},
    keyboard::Keycode as k,
};

impl Interface {
    pub fn listen(&mut self, road: &mut Road) -> Result<(), String> {
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
                } => road.new_vehicle(Direction::North),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::DOWN),
                    ..
                } => road.new_vehicle(Direction::South),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::RIGHT),
                    ..
                } => road.new_vehicle(Direction::East),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::LEFT),
                    ..
                } => road.new_vehicle(Direction::West),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::R),
                    ..
                } => road.new_vehicle(Direction::random()),
                //__________________________________
                _ => {}
            }
        }

        Ok(())
    }
}
