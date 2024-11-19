use super::Interface;
use crate::models::{Direction, Intersection};
use sdl2::{
    event::Event::{KeyDown, Quit},
    keyboard::Keycode as k,
};

impl Interface {
    pub fn listen(&mut self, intersection: &mut Intersection) -> Result<(), String> {
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
                } => intersection.new_vehicle(Direction::North),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::DOWN),
                    ..
                } => intersection.new_vehicle(Direction::South),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::RIGHT),
                    ..
                } => intersection.new_vehicle(Direction::East),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::LEFT),
                    ..
                } => intersection.new_vehicle(Direction::West),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::R),
                    ..
                } => intersection.new_vehicle(Direction::random()),
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::S),
                    ..
                } => intersection.sensor_visibility = !intersection.sensor_visibility,
                //__________________________________________________
                KeyDown {
                    keycode: Some(k::A),
                    ..
                } => intersection.auto_spawn = !intersection.auto_spawn,
                //__________________________________
                _ => {}
            }
        }

        Ok(())
    }
}
