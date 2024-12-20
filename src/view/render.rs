use super::Interface;
use crate::models::Intersection;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;

impl Interface {
    /// This function is responsible for rendering
    /// everything that has been drawn on the canvas
    /// by calling the concerned drawing functions.
    pub fn render(&mut self, intersection: &Intersection) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        let intersection_texture = self
            .texture_creator
            .load_texture("./assets/img/intersection.png")?;

        self.canvas.copy(&intersection_texture, None, None)?;

        self.canvas.set_draw_color(Color::GREEN);

        intersection
            .sensors
            .iter()
            .flat_map(|tab| tab.iter())
            .for_each(|sensor| self.canvas.draw_point(*sensor).unwrap());

        let vehicle_texture = self
            .texture_creator
            .load_texture("./assets/img/vehicle.png")?;

        for vehicle in &intersection.vehicles {
            if intersection.sensor_visibility && vehicle.into_area(&intersection.collision_area) {
                self.canvas.draw_rect(vehicle.sensor_range())?;
            };

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
