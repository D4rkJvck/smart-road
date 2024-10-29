use crate::models::{Road, Vehicle};
use super::Interface;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture; // Import pour permettre le chargement des textures
use std::{thread, time::Duration};

impl Interface {
    /// Cette fonction est responsable de rendre
    /// tout ce qui a été dessiné sur le canvas
    /// en appelant les fonctions de dessin concernées.
    pub fn render(&mut self, road: &Road, vehicles: &Vec<Vehicle>) -> Result<(), String> {
        // Effacer le canvas avec une couleur de fond noire
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        // Dessiner les lignes de la route en blanc
        self.canvas.set_draw_color(Color::WHITE);
        for line in &road.lines {
            self.canvas.draw_line(line.start, line.end)?;
        }

        // Charger la texture du véhicule depuis un fichier PNG
        let texture_creator = self.canvas.texture_creator();
        let vehicle_texture = texture_creator.load_texture("./assets/cars.png")?;

        // Facteur de mise à l'échelle
        let scale_factor = 0.5; // Réduire la taille à 50%

        // Dessiner chaque véhicule en vert et appliquer la texture du véhicule
        self.canvas.set_draw_color(Color::GREEN);
        for vehicle in vehicles {
            self.canvas.draw_rect(vehicle.area)?;

            // Calculer le rectangle de destination avec la taille réduite
            let scaled_area = sdl2::rect::Rect::new(
                vehicle.area.x + 5,
                vehicle.area.y + 15,
                (vehicle.area.width() as f32 * scale_factor) as u32 + 10,
                (vehicle.area.height() as f32 * scale_factor) as u32,
            );

            // Appliquer la texture à la zone redimensionnée
            self.canvas.copy(&vehicle_texture, None, scaled_area)?;
        }

        // Afficher le canvas sur l'écran
        self.canvas.present();
        thread::sleep(Duration::from_millis(16));

        Ok(())
    }
}
