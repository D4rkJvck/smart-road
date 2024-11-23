use super::App;
use crate::{models::Direction, view::Interface};
use sdl2::{
    event::Event::{KeyDown, Quit},
    keyboard::Keycode as k,
};

impl App {
    pub fn listen(&mut self) -> Result<(), String> {
        let events = self.window.event_pump.poll_iter();

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
                } => self.intersection.new_vehicle(Direction::North),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::DOWN),
                    ..
                } => self.intersection.new_vehicle(Direction::South),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::RIGHT),
                    ..
                } => self.intersection.new_vehicle(Direction::East),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::LEFT),
                    ..
                } => self.intersection.new_vehicle(Direction::West),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::R),
                    ..
                } => self.intersection.new_vehicle(Direction::random()),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::S),
                    ..
                } => self.intersection.sensor_visibility = !self.intersection.sensor_visibility,
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::A),
                    ..
                } => self.intersection.auto_spawn = !self.intersection.auto_spawn,
                //__________________________________
                _ => {}
            }
        }

        Ok(())
    }
}
