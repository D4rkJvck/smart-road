use sdl2::{
    event::Event::{KeyDown, Quit},
    keyboard::Keycode,
};

use super::Interface;

impl Interface {
    pub fn listen(&mut self) -> Result<(), String> {
        let events = self.event_pump.poll_iter();

        for event in events {
            match event {
                Quit { .. }
                | KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => return Err("Exiting...".to_string()),
                _ => {}
            }
        }

        Ok(())
    }
}
