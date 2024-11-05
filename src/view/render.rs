use super::Interface;
use crate::models::{Road, Vehicle};
use sdl2::image::LoadTexture; // Import pour permettre le chargement des textures
use sdl2::pixels::Color;

impl Interface {
    /// This function is responsible for rendering
    /// everything that has been drawn on the canvas
    /// by calling the concerned drawing functions.
    pub fn render(&mut self, road: &Road, vehicles: &Vec<Vehicle>) -> Result<(), String> {
        // Effacer le canvas avec une couleur de fond noire
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        let road_texture = self.texture_creator.load_texture("./assets/road.png")?;
        self.canvas.copy(&road_texture, None, None)?;

        self.canvas.set_draw_color(Color::WHITE);
        // for line in &road.lines {
        //     self.canvas.draw_line(line.start, line.end)?;
        // }

        road.intersection
            .sensors
            .iter()
            .flat_map(|tab| tab.iter())
            .for_each(|rect| self.canvas.draw_rect(*rect).unwrap());

        // self.canvas.set_draw_color(Color::BLACK);
        // self.canvas.fill_rect(road.intersection.area)?;

        self.canvas.set_draw_color(Color::GREEN);
        for vehicle in vehicles {
            // self.canvas.draw_rect(vehicle.area)?;
            let vehicle_texture = self.texture_creator.load_texture(vehicle.category())?;
            self.canvas.copy_ex(
                &vehicle_texture,
                None,
                vehicle.area,
                vehicle.angle(),
                None,
                false,
                false,
            )?;
        }

        Ok(self.canvas.present())
    }
}
