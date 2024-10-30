use super::Interface;
use crate::models::{Road, Vehicle};
use sdl2::image::LoadTexture; // Import pour permettre le chargement des textures
use sdl2::pixels::Color;
use std::{thread, time::Duration};

impl Interface {
    /// This function is responsible for rendering
    /// everything that has been drawn on the canvas
    /// by calling the concerned drawing functions.
    pub fn render(&mut self, road: &Road, vehicles: &Vec<Vehicle>) -> Result<(), String> {
        // Effacer le canvas avec une couleur de fond noire
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        // Dessiner les lignes de la route en blanc
        self.canvas.set_draw_color(Color::WHITE);
        for line in &road.lines {
            self.canvas.draw_line(line.start, line.end)?;
        }

        // Dessiner chaque véhicule en vert et appliquer la texture du véhicule
        self.canvas.set_draw_color(Color::GREEN);
        for vehicle in vehicles {
            self.canvas.draw_rect(vehicle.area)?;
            let vehicle_texture = self.texture_creator.load_texture(vehicle.img_path)?;
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

        // Afficher le canvas sur l'écran
        self.canvas.present();
        thread::sleep(Duration::from_millis(16));

        Ok(())
    }
}
