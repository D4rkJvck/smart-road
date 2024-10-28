use super::Window;
use crate::models::{Road, Vehicle};
use sdl2::pixels::Color;
use std::{thread, time::Duration};

pub struct Data<'a> {
    pub road: &'a Road,
    pub vehicles: &'a Vec<Vehicle>,
}

impl Window {
    /// This function is responsible for rendering
    /// everything that has been drawn on the canvas
    /// by calling the concerned drawing functions.
    pub fn render(&mut self, data: Data) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);
        data.road
            .lines
            .iter()
            .for_each(|line| self.canvas.draw_line(line.start, line.end).unwrap());

        self.canvas.present();
        thread::sleep(Duration::from_millis(16));

        Ok(())
    }
}
