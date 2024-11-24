use super::Vehicle;
use std::{time::Duration, u32, u64};

pub struct Stats {
    vehicle_count: usize,
    max_speed: u64,
    min_speed: u64,
    max_time: Duration,
    min_time: Duration,
    pub collisions: u32,
    pub close_calls: u32,
    pub priority_calls: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            vehicle_count: 0,
            max_speed: 0,
            min_speed: u64::MAX,
            max_time: Duration::from_secs(0),
            min_time: Duration::from_secs(u64::MAX),
            collisions: 0,
            close_calls: 0,
            priority_calls: 0,
        }
    }

    pub fn get(&self) -> Vec<String> {
        vec![
            String::from("STATISTICS"),
            format!("Vehicle Passed: {}", self.vehicle_count),
            format!("Max Velocity: {} pixels/second", self.max_speed),
            format!("Min Velocity: {} pixels/second", self.min_speed),
            format!("Max Time: {:.2} seconds", self.max_time.as_secs()),
            format!("Min Time: {:.2} seconds", self.min_time.as_secs()),
            format!("Collisions: {}", self.collisions),
            format!("Close calls: {}", self.close_calls),
            format!("Priority calls: {}", self.priority_calls),
        ]
    }

    pub fn update(&mut self, vehicles: &Vec<Vehicle>) {
        let vehicles: Vec<&Vehicle> = vehicles
            .iter()
            .filter(|vehicle| !vehicle.is_visible())
            .collect();

        self.vehicle_count += vehicles.iter().count();

        for vehicle in vehicles {
            let elapsed = vehicle.time.elapsed();
            let speed = vehicle.distance / elapsed.as_secs();

            if elapsed > self.max_time {
                self.max_time = elapsed;
            }

            if elapsed < self.min_time {
                self.min_time = elapsed;
            }

            if speed > self.max_speed {
                self.max_speed = speed;
            }

            if speed < self.min_speed {
                self.min_speed = speed;
            }
        }
    }
}
