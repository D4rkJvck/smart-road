use super::Interface;
use crate::{HEIGHT, WIDTH};
use sdl2::{pixels::Color, rect::Rect, video::WindowPos};
use std::path::Path;

impl Interface {
    pub fn display_stats(&mut self, stats: Vec<String>) -> Result<(), String> {
        self.shrink()?;

        let font = self
            .ttf_ctx
            .load_font(Path::new("./assets/fonts/Doto-Bold.ttf"), 16)
            .map_err(|err| format!("Font! -> {}", err))?;

        let mut vertical_offset = 40;

        self.canvas.set_draw_color(Color::WHITE);

        for stat in stats {
            let surface = font
                .render(&stat)
                .blended(Color::WHITE)
                .map_err(|err| format!("Surface! -> {}", err))?;

            let texture = self
                .texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|err| format!("Texture! -> {}", err))?;

            let txt_query = texture.query();
            let area = Rect::new(60, vertical_offset, txt_query.width, txt_query.height);

            self.canvas.copy(&texture, None, Some(area))?;

            vertical_offset += 40;
        }

        Ok(self.canvas.present())
    }

    /// This method reduces the size of the window
    /// by half, then centers it on the screen.
    fn shrink(&mut self) -> Result<(), String> {
        self.canvas
            .window_mut()
            .set_size(WIDTH as u32 / 2, HEIGHT as u32 / 2)
            .map_err(|err| format!("Resize! -> {}", err))?;

        self.canvas
            .window_mut()
            .set_position(WindowPos::Centered, WindowPos::Centered);

        Ok(())
    }
}
