use super::Window;
use sdl2::pixels::Color;
use std::{thread, time::Duration};

impl Window {
    /// This function is responsible for rendering
    /// everything that has been drawn on the canvas
    /// by calling the concerned drawing functions.
    pub fn render(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        

        self.canvas.present();
        thread::sleep(Duration::from_millis(16));

        Ok(())
    }
}
