use crate::model::{Road, Vehicle};

use super::Interface;
use sdl2::pixels::Color;
use std::{thread, time::Duration};

impl Interface {
    /// This function is responsible for rendering
    /// everything that has been drawn on the canvas
    /// by calling the concerned drawing functions.
    pub fn render(&mut self, road: &Road, vehicles: &Vec<Vehicle>) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);

        for line in &road.lines {
            self.canvas.draw_line(line.start, line.end)?;
        }
        
        self.canvas.present();
        thread::sleep(Duration::from_millis(16));

        Ok(())
    }
}
