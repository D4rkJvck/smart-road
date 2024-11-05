use super::Vehicle;
use crate::{HEIGHT, SAFETY_DISTANCE, WIDTH};
use sdl2::rect::Rect;

impl Vehicle {
    /// This function is crucial when
    /// it comes to remove vehicule
    /// as it confirm that the
    /// vehicle is out of the window.
    pub fn is_visible(&self) -> bool {
        self.area.top() <= HEIGHT as i32
            && self.area.left() <= WIDTH as i32
            && self.area.right() >= 0
            && self.area.bottom() >= 0
    }

    pub fn into_area(&self, area: &Rect) -> bool {
        self.area.right() > area.left()
            && self.area.left() < area.right()
            && self.area.top() < area.bottom()
            && self.area.bottom() > area.top()
    }

    pub fn too_close_to(&self, other: &Self) -> bool {
        let diff_x = self.area.center().x - other.area.center().x;
        let diff_y = self.area.center().y - other.area.center().y;

        let distance = diff_x * diff_y;

        self.direction == other.direction && distance <= SAFETY_DISTANCE
    }

    pub fn has_priority_over(&self, other: &Self) -> bool {
        // Si le véhicule actuel a une priorité plus élevée (numéro plus bas) que l'autre
        if self.priority < other.priority {
            return true;
        }

        // Si les deux véhicules ont la même priorité, on vérifie la distance de sécurité
        if self.priority == other.priority && !self.too_close_to(other) {
            return true;
        }

        false
    }
}
