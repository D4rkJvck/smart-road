use super::{Direction, Speed, Vehicle};

impl Vehicle {
    /// The velicity method gives
    /// the amount of number of
    /// pixels for the vehicle
    /// to translate.
    pub fn velocity(&self) -> i32 {
        match self.speed {
            Speed::Fast => 3,
            Speed::Normal => 2,
            Speed::Slow => 1,
            Speed::Stop => 0,
        }
    }

    /// This method returns the
    /// rotation angle of the
    /// vehicle's picture
    /// depending on the direction.
    pub fn angle(&self) -> f64 {
        match self.direction {
            Direction::North => 0.0,
            Direction::East => 90.0,
            Direction::South => 180.0,
            Direction::West => 270.0,
        }
    }
}
