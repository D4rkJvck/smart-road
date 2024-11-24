use super::Vehicle;
use std::time::Duration;

pub struct Stats {
    vehicle_count: usize,
    max_speed: f32,
    min_speed: f32,
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
            max_speed: 0.0,
            min_speed: 0.0,
            max_time: Duration::from_secs_f32(0.0),
            min_time: Duration::from_secs_f32(0.0),
            collisions: 0,
            close_calls: 0,
            priority_calls: 0,
        }
    }

    pub fn get(&self) -> Vec<String> {
        vec![
            String::from("STATISTICS"),
            format!("Vehicle Passed:    {}", self.vehicle_count),
            format!(
                "Max Velocity:      {} m/s",
                (self.max_speed * 10.0).round() / 100.0
            ),
            format!(
                "Min Velocity:      {} m/s",
                (self.min_speed * 10.0).round() / 100.0
            ),
            format!("Max Time:          {:.2} s", self.max_time.as_secs_f32()),
            format!("Min Time:          {:.2} s", self.min_time.as_secs_f32()),
            format!("Collisions:        {}", self.collisions),
            format!("Close calls:       {}", self.close_calls),
            format!("Priority calls:    {}", self.priority_calls),
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
            let speed = vehicle.distance / elapsed.as_secs_f32();

            if elapsed > self.max_time {
                self.max_time = elapsed;
            }

            if self.min_time == Duration::from_secs_f32(0.0) || elapsed < self.min_time {
                self.min_time = elapsed;
            }

            if speed > self.max_speed {
                self.max_speed = speed;
            }

            if self.min_speed == 0.0 || speed < self.min_speed {
                self.min_speed = speed;
            }
        }
    }
}
