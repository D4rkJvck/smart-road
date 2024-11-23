use super::Vehicle;

pub struct Stats {
    vehicle_count: usize,
    max_speed: u32,
    min_speed: u32,
    max_time: f32,
    min_time: f32,
    collisions: u32,
    close_calls: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            vehicle_count: 0,
            max_speed: 0,
            min_speed: u32::MAX,
            max_time: 0.0,
            min_time: 0.0,
            collisions: 0,
            close_calls: 0,
        }
    }

    pub fn get(&self) -> Vec<String> {
        vec![
            format!("Vehicle Passed: {}", self.vehicle_count),
            format!("Max Velocity: {}", self.max_speed),
            format!("Min Velocity: {}", self.min_speed),
            format!("Max Time: {:.2} seconds", self.max_time),
            format!("Min Time: {:.2} seconds", self.min_time),
            format!("Close call: {}", self.close_calls),
            format!("Collisions: {}", self.collisions),
        ]
    }

    pub fn update(&mut self, vehicles: &Vec<Vehicle>) {
        let vehicles: Vec<&Vehicle> = vehicles
            .iter()
            .filter(|vehicle| !vehicle.is_visible())
            .collect();

        self.vehicle_count += vehicles.iter().count();
    }
}
